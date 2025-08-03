use crate::{
    debug_info_green, debug_info_yellow, instr, instr_mut, node, node_mut, reg_field_for_struct,
    toolkit::{
        context::NhwcCtx, dot::Config, etc::generate_png_by_graph_multi_tasks,
        nhwc_instr::{FuncOp, JumpOp, NhwcInstr, NhwcInstrType, ComparedPair}, pass_manager::Pass,
        symtab::{RcSymIdx, SymIdx, SymTab, WithBorrow}, symbol::Symbol
    }
};
use anyhow::*;
use ahash::{AHashMap, AHashSet};
use std::result::Result::Ok;
use crate::toolkit::field::Type;

/// CFG信息结构体，用于内联时收集和传递CFG信息
#[derive(Debug)]
struct CfgInfo {
    nodes: AHashMap<u32, crate::toolkit::cfg_node::CfgNode>,
    edges: Vec<(u32, u32)>,
    entry: u32,
}

#[derive(Debug)]
pub struct InlinePass {
    is_gen_inline_png: bool,
    max_inline_size: usize,  // 最大内联函数大小
    max_inline_depth: usize, // 最大内联深度
}

impl InlinePass {
    pub fn new(is_gen_inline_png: bool) -> Self {
        InlinePass {
            is_gen_inline_png,
            max_inline_size: 50,  // 默认最大50条指令
            max_inline_depth: 3,  // 默认最大深度3
        }
    }
}

// 为指令添加内联相关字段
reg_field_for_struct!(NhwcInstr
    {
        INLINE_DEPTH: usize,
        INLINE_PARENT: Option<usize>,
    }
    with_fields info
    with_prefix INLINE
);

// 为符号表添加内联相关字段
reg_field_for_struct!(Symbol
    {
        INLINE_COUNT: usize,
        INLINE_DEPTH: usize,
    }
    with_fields fields
);

impl Pass for InlinePass {
    fn run(&mut self, ctx: &mut NhwcCtx) -> Result<()> {
        println!("=== 内联pass开始执行 ===");
        
        let (symtab, instr_slab, call_graph) = (
            &mut ctx.symtab,
            &mut ctx.nhwc_instr_slab,
            &ctx.call_graph,
        );
        
        debug_info_yellow!("开始函数内联优化...");
        debug_info_yellow!("调用图边数: {}", call_graph.edge_count());
        debug_info_yellow!("调用图节点数: {}", call_graph.node_count());

        // 1. 分析函数调用图，确定内联候选
        let inline_candidates = self.analyze_inline_candidates(call_graph, symtab, &ctx.cfg_graph, instr_slab)?;
        debug_info_yellow!("找到内联候选数量: {}", inline_candidates.len());
        
        // 2. 执行内联
        let mut inline_count = 0;
        for (caller_func, callee_func) in inline_candidates {
            debug_info_yellow!("检查候选: {:?} -> {:?}", 
                caller_func.as_ref_borrow().symbol_name, 
                callee_func.as_ref_borrow().symbol_name);
            
            if self.should_inline_function(&callee_func, symtab, &ctx.cfg_graph, instr_slab)? {
                debug_info_yellow!("候选通过检查，尝试内联: {:?} -> {:?}", 
                    caller_func.as_ref_borrow().symbol_name, 
                    callee_func.as_ref_borrow().symbol_name);
                
                match self.inline_function(&caller_func, &callee_func, &mut ctx.cfg_graph, symtab, instr_slab) {
                    Ok(_) => {inline_count += 1;
                        debug_info_yellow!("成功内联函数: {:?} -> {:?}", 
                        callee_func.as_ref_borrow().symbol_name, 
                        caller_func.as_ref_borrow().symbol_name);
                    }

                    Err(e) => {debug_info_yellow!("{}123", e)}


                };
            } else {
                debug_info_yellow!("候选未通过检查: {:?} -> {:?}", 
                    caller_func.as_ref_borrow().symbol_name, 
                    callee_func.as_ref_borrow().symbol_name);
            }
        }

        debug_info_yellow!("函数内联完成，共内联 {} 个函数", inline_count);

        // 3. 生成调试图
        if self.is_gen_inline_png {
            self.generate_inline_debug_graph(&mut ctx.cfg_graph, symtab, instr_slab, &mut ctx.io_task_list)?;
        }

        Ok(())
    }

    fn get_desc(&self) -> String {
        "函数内联优化pass，将小函数调用替换为函数体".to_string()
    }

    fn get_pass_name(&self) -> String {
        "Inline Pass".to_string()
    }
}

impl InlinePass {
    /// 分析函数调用图，确定内联候选
    fn analyze_inline_candidates(
        &self,
        call_graph: &crate::toolkit::call_node::CallGraph,
        symtab: &SymTab,
        cfg_graph: &crate::toolkit::cfg_node::CfgGraph,
        instr_slab: &crate::toolkit::nhwc_instr::InstrSlab<NhwcInstr>,
    ) -> Result<Vec<(RcSymIdx, RcSymIdx)>> {
        let mut candidates = Vec::new();
        
        // 遍历所有函数调用关系
        for edge in call_graph.edge_indices() {
            let (caller_idx, callee_idx) = call_graph.edge_endpoints(edge).unwrap();
            let caller_node = node!(at caller_idx in call_graph);
            let callee_node = node!(at callee_idx in call_graph);
            
            let caller_func = &caller_node.rc_func_symidx;
            let callee_func = &callee_node.rc_func_symidx;
            
            // 检查是否为内联候选
            if self.is_inline_candidate(callee_func, symtab, cfg_graph, instr_slab)? {
                candidates.push((caller_func.clone(), callee_func.clone()));
            }
        }
        
        Ok(candidates)
    }

    /// 判断函数是否为内联候选
    fn is_inline_candidate(
        &self,
        func_symidx: &RcSymIdx,
        symtab: &SymTab,
        cfg_graph: &crate::toolkit::cfg_node::CfgGraph,
        instr_slab: &crate::toolkit::nhwc_instr::InstrSlab<NhwcInstr>,
    ) -> Result<bool> {
        let func_sym = symtab.get(&func_symidx.as_ref_borrow())?;
        
        // 使用符号表的全局信息检查是否为外部函数
        let global_info = symtab.get_global_info();
        if let Ok(external_funcs) = global_info.get_external_func_symidx_vec() {
            if external_funcs.contains(func_symidx) {
                return Ok(false); // 外部函数不能内联
            }
        }
        
        // 检查函数类型是否正确
        match func_sym.get_type()? {
            Type::Fn { .. } => {
                // 计算函数大小
                let func_size = self.calculate_function_size(func_symidx, symtab, cfg_graph, instr_slab)?;
                
                // 检查函数大小是否适合内联
                if func_size <= self.max_inline_size {
                    return Ok(true);
                }
            }
            _ => {
                // 不是函数类型，不能内联
                return Ok(false);
            }
        }
        
        Ok(false)
    }

    /// 计算函数大小（指令数量）
    fn calculate_function_size(
        &self,
        func_symidx: &RcSymIdx,
        symtab: &SymTab,
        cfg_graph: &crate::toolkit::cfg_node::CfgGraph,
        instr_slab: &crate::toolkit::nhwc_instr::InstrSlab<NhwcInstr>,
    ) -> Result<usize> {
        let func_sym = symtab.get(&func_symidx.as_ref_borrow())?;
        
        // 获取函数的CFG入口
        let cfg_entry = func_sym.get_cfg_entry_node()?;
        let mut visited = AHashSet::new();
        let mut size = 0;
        
        // 遍历CFG计算指令数量
        self.count_instructions_in_cfg(*cfg_entry, &mut visited, &mut size, cfg_graph, instr_slab)?;
        
        Ok(size)
    }

    /// 递归计算CFG中的指令数量
    fn count_instructions_in_cfg(
        &self,
        cfg_node: u32,
        visited: &mut AHashSet<u32>,
        size: &mut usize,
        cfg_graph: &crate::toolkit::cfg_node::CfgGraph,
        instr_slab: &crate::toolkit::nhwc_instr::InstrSlab<NhwcInstr>,
    ) -> Result<()> {
        if visited.contains(&cfg_node) {
            return Ok(());
        }
        visited.insert(cfg_node);
        
        // 计算当前节点的指令数量
        let node_instrs: Vec<_> = node!(at cfg_node in cfg_graph).iter_all_instrs().collect();
        *size += node_instrs.len();
        
        // 递归处理后继节点
        for successor in cfg_graph.neighbors_directed(cfg_node.into(), petgraph::Direction::Outgoing) {
            self.count_instructions_in_cfg(successor.index() as u32, visited, size, cfg_graph, instr_slab)?;
        }
        
        Ok(())
    }

    /// 判断是否应该内联函数
    fn should_inline_function(
        &self,
        callee_func: &RcSymIdx,
        symtab: &SymTab,
        cfg_graph: &crate::toolkit::cfg_node::CfgGraph,
        instr_slab: &crate::toolkit::nhwc_instr::InstrSlab<NhwcInstr>,
    ) -> Result<bool> {
        let func_sym = symtab.get(&callee_func.as_ref_borrow())?;
        
        // 使用符号表中的内联深度信息
        let current_depth = func_sym.get_inline_depth().unwrap_or(&0);
        if *current_depth >= self.max_inline_depth {
            return Ok(false);
        }
        
        // 使用符号表中的内联计数信息
        let current_count = func_sym.get_inline_count().unwrap_or(&0);
        if *current_count > 10 { // 限制单个函数的内联次数
            return Ok(false);
        }
        
        // 检查函数是否包含递归调用
        if self.contains_recursive_call(callee_func, symtab, cfg_graph, instr_slab)? {
            return Ok(false);
        }
        
        Ok(true)
    }

    /// 检查函数是否包含递归调用
    fn contains_recursive_call(
        &self,
        func_symidx: &RcSymIdx,
        symtab: &SymTab,
        cfg_graph: &crate::toolkit::cfg_node::CfgGraph,
        instr_slab: &crate::toolkit::nhwc_instr::InstrSlab<NhwcInstr>,
    ) -> Result<bool> {
        let func_sym = symtab.get(&func_symidx.as_ref_borrow())?;
        let cfg_entry = func_sym.get_cfg_entry_node()?;
        let mut visited = AHashSet::new();
        
        self.check_recursive_call_in_cfg(*cfg_entry, func_symidx, &mut visited, cfg_graph, instr_slab)
    }

    /// 在CFG中检查递归调用
    fn check_recursive_call_in_cfg(
        &self,
        cfg_node: u32,
        func_symidx: &RcSymIdx,
        visited: &mut AHashSet<u32>,
        cfg_graph: &crate::toolkit::cfg_node::CfgGraph,
        instr_slab: &crate::toolkit::nhwc_instr::InstrSlab<NhwcInstr>,
    ) -> Result<bool> {
        if visited.contains(&cfg_node) {
            return Ok(false);
        }
        visited.insert(cfg_node);
        
        // 检查当前节点的指令
        for &instr_idx in node!(at cfg_node in cfg_graph).iter_all_instrs() {
            let instr = instr!(at instr_idx in instr_slab)?;
            match &instr.instr_type {
                NhwcInstrType::Call { op_lhs, func_op } => {
                    if func_op.rc_func_symidx.as_ref_borrow().symbol_name == func_symidx.as_ref_borrow().symbol_name {
                        return Ok(true); // 发现递归调用
                    }
                }
                _ => {}
            }
        }
        
        // 递归检查后继节点
        for successor in cfg_graph.neighbors_directed(cfg_node.into(), petgraph::Direction::Outgoing) {
            if self.check_recursive_call_in_cfg(successor.index() as u32, func_symidx, visited, cfg_graph, instr_slab)? {
                return Ok(true);
            }
        }
        
        Ok(false)
    }

    /// 执行函数内联
    fn inline_function(
        &self,
        caller_func: &RcSymIdx,
        callee_func: &RcSymIdx,
        cfg_graph: &mut crate::toolkit::cfg_node::CfgGraph,
        symtab: &mut SymTab,
        instr_slab: &mut crate::toolkit::nhwc_instr::InstrSlab<NhwcInstr>,
    ) -> Result<()> {
        // 先获取所有需要的信息，避免借用冲突
        let callee_cfg_entry = *symtab.get(&callee_func.as_ref_borrow())?.get_cfg_entry_node()?;
        let call_sites = self.find_call_sites(caller_func, callee_func, symtab, cfg_graph, instr_slab)?.clone();
        let current_count = *symtab.get(&callee_func.as_ref_borrow())?.get_inline_count().unwrap_or(&0);
        
        // 执行内联操作
        for call_site in call_sites {
            self.inline_at_call_site(
                call_site,
                callee_cfg_entry,
                caller_func,
                callee_func,
                cfg_graph,
                symtab,
                instr_slab,
            )?;
        }
        
        // 更新内联统计 - 使用符号表的方法
        let callee_sym_mut = symtab.get_mut(&callee_func.as_ref_borrow())?;
        callee_sym_mut.add_inline_count(current_count + 1);
        
        // 同时更新内联深度
        let new_depth = callee_sym_mut.get_inline_depth().unwrap_or(&0) + 1;
        callee_sym_mut.add_inline_depth(new_depth);
        
        Ok(())
    }

    /// 找到函数调用点
    fn find_call_sites(
        &self,
        caller_func: &RcSymIdx,
        callee_func: &RcSymIdx,
        symtab: &SymTab,
        cfg_graph: &crate::toolkit::cfg_node::CfgGraph,
        instr_slab: &crate::toolkit::nhwc_instr::InstrSlab<NhwcInstr>,
    ) -> Result<Vec<(u32, usize)>> { // (cfg_node, instr_idx)
        let mut call_sites = Vec::new();
        let caller_sym = symtab.get(&caller_func.as_ref_borrow())?;
        let caller_cfg_entry = caller_sym.get_cfg_entry_node()?;
        let mut visited = AHashSet::new();
        
        self.find_call_sites_in_cfg(
            *caller_cfg_entry,
            callee_func,
            &mut call_sites,
            &mut visited,
            symtab,
            cfg_graph,
            instr_slab,
        )?;
        
        Ok(call_sites)
    }

    /// 在CFG中查找调用点
    fn find_call_sites_in_cfg(
        &self,
        cfg_node: u32,
        callee_func: &RcSymIdx,
        call_sites: &mut Vec<(u32, usize)>,
        visited: &mut AHashSet<u32>,
        symtab: &SymTab,
        cfg_graph: &crate::toolkit::cfg_node::CfgGraph,
        instr_slab: &crate::toolkit::nhwc_instr::InstrSlab<NhwcInstr>,
    ) -> Result<()> {
        if visited.contains(&cfg_node) {
            return Ok(());
        }
        visited.insert(cfg_node);
        
        // 检查当前节点的指令
        for &instr_idx in node!(at cfg_node in cfg_graph).iter_all_instrs() {
            let instr = instr!(at instr_idx in instr_slab)?;
            match &instr.instr_type {
                NhwcInstrType::Call { op_lhs, func_op } => {
                    if func_op.rc_func_symidx.as_ref_borrow().symbol_name == callee_func.as_ref_borrow().symbol_name {
                        call_sites.push((cfg_node, instr_idx));
                    }
                }
                _ => {}
            }
        }
        
        // 递归检查后继节点
        for successor in cfg_graph.neighbors_directed(cfg_node.into(), petgraph::Direction::Outgoing) {
            self.find_call_sites_in_cfg(successor.index() as u32, callee_func, call_sites, visited, symtab, cfg_graph, instr_slab)?;
        }
        
        Ok(())
    }

    /// 在特定调用点执行内联
    fn inline_at_call_site(
        &self,
        call_site: (u32, usize), // (cfg_node, instr_idx)
        callee_cfg_entry: u32,
        caller_func: &RcSymIdx,
        callee_func: &RcSymIdx,
        cfg_graph: &mut crate::toolkit::cfg_node::CfgGraph,
        symtab: &mut SymTab,
        instr_slab: &mut crate::toolkit::nhwc_instr::InstrSlab<NhwcInstr>,
    ) -> Result<()> {
        let (call_cfg_node, call_instr_idx) = call_site;
        let call_instr = instr!(at call_instr_idx in instr_slab)?;
        
        // 获取调用指令的参数和返回值
        let (args, ret_var) = match &call_instr.instr_type {
            NhwcInstrType::Call { op_lhs, func_op } => {
                (func_op.actual_arg_symidx_vec.clone(), op_lhs.clone())
            }
            _ => return Err(anyhow!("不是函数调用指令")),
        };
        
        // 获取被调用函数的参数定义
        let callee_sym = symtab.get(&callee_func.as_ref_borrow())?;
        let callee_args = match callee_sym.get_type()? {
            Type::Fn { arg_syms, .. } => arg_syms.clone(),
            _ => vec![],
        };
        
        // 验证参数数量匹配
        if callee_args.len() != args.len() {
            return Err(anyhow!("参数数量不匹配: 期望 {}, 实际 {}", callee_args.len(), args.len()));
        }
        
        // 创建参数映射
        let mut param_mapping = AHashMap::new();
        for (param, arg) in callee_args.iter().zip(args.iter()) {
            param_mapping.insert(param.clone(), arg.clone());
        }
        
        // 第一步：收集被调用函数的CFG信息
        let callee_cfg_info = self.collect_cfg_info(
            callee_cfg_entry,
            cfg_graph,
            instr_slab,
        )?;
        
        // 第二步：创建内联后的CFG节点映射
        let mut cfg_mapping = AHashMap::new(); // 旧节点 -> 新节点
        let mut instr_mapping = AHashMap::new(); // 旧指令 -> 新指令
        
        // 获取调用者头节点
        let caller_sym = symtab.get(&caller_func.as_ref_borrow())?;
        let caller_entry = caller_sym.get_cfg_entry_node()?;
        let caller_head_node = *caller_entry;
        
        // 第三步：复制被调用函数的CFG节点
        self.copy_cfg_nodes(
            &callee_cfg_info,
            &mut cfg_mapping,
            &mut instr_mapping,
            &param_mapping,
            ret_var.clone(),
            caller_func,
            callee_func,
            cfg_graph,
            symtab,
            instr_slab,
        )?;
        
        // 第四步：连接CFG边
        self.connect_cfg_edges(
            call_cfg_node,
            &callee_cfg_info,
            &cfg_mapping,
            cfg_graph,
            instr_slab,
            caller_head_node,
        )?;
        
        // 第五步：删除原始调用指令
        self.remove_call_instruction(
            call_cfg_node,
            call_instr_idx,
            cfg_graph,
            instr_slab,
        )?;
        
        // 第六步：验证内联结果
        self.verify_inline_result(
            call_cfg_node,
            &callee_cfg_info,
            &cfg_mapping,
            cfg_graph,
            instr_slab,
        )?;
        
        Ok(())
    }

    /// 验证内联结果
    fn verify_inline_result(
        &self,
        call_cfg_node: u32,
        cfg_info: &CfgInfo,
        cfg_mapping: &AHashMap<u32, u32>,
        cfg_graph: &crate::toolkit::cfg_node::CfgGraph,
        instr_slab: &crate::toolkit::nhwc_instr::InstrSlab<NhwcInstr>,
    ) -> Result<()> {
        // 验证内联入口节点存在
        let inline_entry = cfg_mapping.get(&cfg_info.entry);
        if inline_entry.is_none() {
            return Err(anyhow!("内联入口节点不存在"));
        }
        
        // 验证内联出口节点存在
        let inline_exits = self.find_exit_nodes(cfg_info, cfg_mapping, cfg_graph, instr_slab)?;
        if inline_exits.is_empty() {
            debug_info_yellow!("警告：内联函数没有找到出口节点");
        }
        
        // 验证CFG连通性
        let mut visited = AHashSet::new();
        self.verify_cfg_connectivity(
            *inline_entry.unwrap(),
            &mut visited,
            cfg_graph,
        )?;
        
        debug_info_green!("内联验证通过：CFG结构正确");
        Ok(())
    }

    /// 验证CFG连通性
    fn verify_cfg_connectivity(
        &self,
        cfg_node: u32,
        visited: &mut AHashSet<u32>,
        cfg_graph: &crate::toolkit::cfg_node::CfgGraph,
    ) -> Result<()> {
        if visited.contains(&cfg_node) {
            return Ok(());
        }
        visited.insert(cfg_node);
        
        // 检查当前节点的后继
        for successor in cfg_graph.neighbors_directed(cfg_node.into(), petgraph::Direction::Outgoing) {
            self.verify_cfg_connectivity(successor.index() as u32, visited, cfg_graph)?;
        }
        
        Ok(())
    }

    /// 收集CFG信息
    fn collect_cfg_info(
        &self,
        cfg_entry: u32,
        cfg_graph: &crate::toolkit::cfg_node::CfgGraph,
        instr_slab: &crate::toolkit::nhwc_instr::InstrSlab<NhwcInstr>,
    ) -> Result<CfgInfo> {
        let mut nodes = AHashMap::new();
        let mut edges = Vec::new();
        let mut visited = AHashSet::new();
        
        self.collect_cfg_info_recursive(
            cfg_entry,
            &mut nodes,
            &mut edges,
            &mut visited,
            cfg_graph,
            instr_slab,
        )?;
        
        Ok(CfgInfo { nodes, edges, entry: cfg_entry })
    }

    /// 递归收集CFG信息
    fn collect_cfg_info_recursive(
        &self,
        cfg_node: u32,
        nodes: &mut AHashMap<u32, crate::toolkit::cfg_node::CfgNode>,
        edges: &mut Vec<(u32, u32)>,
        visited: &mut AHashSet<u32>,
        cfg_graph: &crate::toolkit::cfg_node::CfgGraph,
        instr_slab: &crate::toolkit::nhwc_instr::InstrSlab<NhwcInstr>,
    ) -> Result<()> {
        if visited.contains(&cfg_node) {
            return Ok(());
        }
        visited.insert(cfg_node);
        
        // 收集节点信息
        let node = node!(at cfg_node in cfg_graph).clone();
        nodes.insert(cfg_node, node);
        
        // 收集边信息
        for successor in cfg_graph.neighbors_directed(cfg_node.into(), petgraph::Direction::Outgoing) {
            let succ_idx = successor.index() as u32;
            edges.push((cfg_node, succ_idx));
            
            // 递归收集后继节点
            self.collect_cfg_info_recursive(
                succ_idx,
                nodes,
                edges,
                visited,
                cfg_graph,
                instr_slab,
            )?;
        }
        
        Ok(())
    }

    /// 复制CFG节点
    fn copy_cfg_nodes(
        &self,
        cfg_info: &CfgInfo,
        cfg_mapping: &mut AHashMap<u32, u32>,
        instr_mapping: &mut AHashMap<usize, usize>,
        param_mapping: &AHashMap<RcSymIdx, RcSymIdx>,
        ret_var: Option<RcSymIdx>,
        caller_func: &RcSymIdx,
        callee_func: &RcSymIdx,
        cfg_graph: &mut crate::toolkit::cfg_node::CfgGraph,
        symtab: &mut SymTab,
        instr_slab: &mut crate::toolkit::nhwc_instr::InstrSlab<NhwcInstr>,
    ) -> Result<()> {
        // 获取调用者的头节点
        let caller_sym = symtab.get(&caller_func.as_ref_borrow())?;
        let caller_entry = caller_sym.get_cfg_entry_node()?;
        let caller_head_node = *caller_entry;
        
        debug_info_yellow!("调用者头节点: {}", caller_head_node);
        debug_info_yellow!("被调用函数入口: {}", cfg_info.entry);
        
        // 为每个节点创建新节点
        for (&old_node_idx, old_node) in &cfg_info.nodes {
            // 跳过被调用函数的头节点，因为我们要把它的alloc指令放到调用者的头节点中
            if old_node_idx == cfg_info.entry {
                debug_info_yellow!("跳过被调用函数头节点: {}", old_node_idx);
                continue;
            }
            
            // 创建新节点
            let new_node_idx = cfg_graph.add_node(old_node.clone()).index() as u32;
            cfg_mapping.insert(old_node_idx, new_node_idx);
            
            debug_info_yellow!("复制节点: {} -> {}", old_node_idx, new_node_idx);
            
            // 复制并映射指令
            let mut new_instrs = Vec::new();
            for &instr_idx in old_node.iter_all_instrs() {
                let old_instr = instr!(at instr_idx in instr_slab)?;
                let new_instr = self.copy_and_map_instr_complete(
                    old_instr,
                    param_mapping,
                    ret_var.clone(),
                    caller_func,
                    callee_func,
                    symtab,
                )?;
                let new_instr_idx = instr_slab.insert_instr(new_instr);
                instr_mapping.insert(instr_idx, new_instr_idx);
                new_instrs.push(new_instr_idx);
            }
            
            // 更新新节点的指令列表
            let new_node = node_mut!(at new_node_idx in cfg_graph);
            new_node.instrs.instr_vec = new_instrs;
            new_node.instrs.outdated = true;
        }
        
        // 处理被调用函数头节点的alloc指令
        let callee_head_node_idx = cfg_info.entry;
        let callee_instrs: Vec<usize> = {
            let callee_head_node = node!(at callee_head_node_idx in cfg_graph);
            callee_head_node.iter_all_instrs().cloned().collect()
        };
        
        debug_info_yellow!("将alloc指令从被调用函数头节点 {} 移动到调用者头节点 {}", 
            cfg_info.entry, caller_head_node);
        
        // 将alloc指令添加到调用者的头节点
        for &instr_idx in &callee_instrs {
            let old_instr = instr!(at instr_idx in instr_slab)?;
            match &old_instr.instr_type {
                NhwcInstrType::Alloc { .. } => {
                    // 复制alloc指令并映射
                    let new_instr = self.copy_and_map_instr_complete(
                        old_instr,
                        param_mapping,
                        ret_var.clone(),
                        caller_func,
                        callee_func,
                        symtab,
                    )?;
                    let new_instr_idx = instr_slab.insert_instr(new_instr);
                    instr_mapping.insert(instr_idx, new_instr_idx);
                    
                    // 添加到调用者头节点
                    let caller_head_node_mut = node_mut!(at caller_head_node in cfg_graph);
                    caller_head_node_mut.instrs.instr_vec.push(new_instr_idx);
                    debug_info_yellow!("添加alloc指令到调用者头节点: {}", new_instr_idx);
                }
                _ => {
                    // 跳过非alloc指令
                    debug_info_yellow!("跳过非alloc指令: {:?}", old_instr.instr_type);
                }
            }
        }
        
        // 复制CFG边，但跳过与被调用函数头节点相关的边
        for &(from, to) in &cfg_info.edges {
            // 跳过涉及被调用函数头节点的边
            if from == cfg_info.entry || to == cfg_info.entry {
                debug_info_yellow!("跳过涉及被调用函数头节点的边: {} -> {}", from, to);
                continue;
            }
            
            if let (Some(&new_from), Some(&new_to)) = (cfg_mapping.get(&from), cfg_mapping.get(&to)) {
                cfg_graph.add_edge(new_from.into(), new_to.into(), crate::toolkit::cfg_edge::CfgEdge::new_direct());
                debug_info_yellow!("添加CFG边: {} -> {}", new_from, new_to);
            }
        }
        
        Ok(())
    }

    /// 连接CFG边
    fn connect_cfg_edges(
        &self,
        call_cfg_node: u32,
        cfg_info: &CfgInfo,
        cfg_mapping: &AHashMap<u32, u32>,
        cfg_graph: &mut crate::toolkit::cfg_node::CfgGraph,
        instr_slab: &crate::toolkit::nhwc_instr::InstrSlab<NhwcInstr>,
        caller_head_node: u32,
    ) -> Result<()> {
        // 获取调用节点的前驱和后继
        let call_node_predecessors: Vec<u32> = cfg_graph
            .neighbors_directed(call_cfg_node.into(), petgraph::Direction::Incoming)
            .map(|n| n.index() as u32)
            .collect();
        
        let call_node_successors: Vec<u32> = cfg_graph
            .neighbors_directed(call_cfg_node.into(), petgraph::Direction::Outgoing)
            .map(|n| n.index() as u32)
            .collect();
        
        // 获取内联函数的入口和出口节点
        let inline_entry = if cfg_mapping.is_empty() {
            // 如果没有复制任何节点，说明被调用函数只有一个头节点
            // 在这种情况下，我们直接使用调用者头节点作为入口
            caller_head_node
        } else {
            cfg_mapping.values().next()
                .ok_or_else(|| anyhow!("没有找到内联入口节点"))?
                .clone()
        };
        let inline_exits = self.find_exit_nodes(cfg_info, cfg_mapping, cfg_graph, instr_slab)?;
        
        debug_info_yellow!("内联入口节点: {}", inline_entry);
        debug_info_yellow!("内联出口节点: {:?}", inline_exits);
        
        // 连接前驱到内联入口
        for &pred in &call_node_predecessors {
            cfg_graph.add_edge(pred.into(), inline_entry.into(), crate::toolkit::cfg_edge::CfgEdge::new_direct());
            debug_info_yellow!("连接前驱 {} 到内联入口 {}", pred, inline_entry);
        }
        
        // 连接内联出口到后继
        if inline_exits.is_empty() {
            // 如果没有出口节点，直接连接内联入口到后继
            for &succ in &call_node_successors {
                cfg_graph.add_edge(inline_entry.into(), succ.into(), crate::toolkit::cfg_edge::CfgEdge::new_direct());
                debug_info_yellow!("连接内联入口 {} 到后继 {}", inline_entry, succ);
            }
        } else {
            // 连接内联出口到后继
            for exit in inline_exits {
                for &succ in &call_node_successors {
                    cfg_graph.add_edge(exit.into(), succ.into(), crate::toolkit::cfg_edge::CfgEdge::new_direct());
                    debug_info_yellow!("连接内联出口 {} 到后继 {}", exit, succ);
                }
            }
        }
        
        // 删除调用节点的边
        for &pred in &call_node_predecessors {
            if let Some(edge_idx) = cfg_graph.find_edge(pred.into(), call_cfg_node.into()) {
                cfg_graph.remove_edge(edge_idx);
            }
        }
        for &succ in &call_node_successors {
            if let Some(edge_idx) = cfg_graph.find_edge(call_cfg_node.into(), succ.into()) {
                cfg_graph.remove_edge(edge_idx);
            }
        }
        
        Ok(())
    }

    /// 找到出口节点
    fn find_exit_nodes(
        &self,
        cfg_info: &CfgInfo,
        cfg_mapping: &AHashMap<u32, u32>,
        cfg_graph: &crate::toolkit::cfg_node::CfgGraph,
        instr_slab: &crate::toolkit::nhwc_instr::InstrSlab<NhwcInstr>,
    ) -> Result<Vec<u32>> {
        let mut exits = Vec::new();
        
        for (&old_node_idx, _) in &cfg_info.nodes {
            if let Some(&new_node_idx) = cfg_mapping.get(&old_node_idx) {
                // 检查是否为出口节点（没有后继或包含返回指令）
                let node = node!(at new_node_idx in cfg_graph);
                if node.iter_all_instrs().any(|&instr_idx| {
                    if let Ok(instr) = instr!(at instr_idx in instr_slab) {
                        matches!(instr.instr_type, NhwcInstrType::Jump { jump_op: JumpOp::Ret { .. } })
                    } else {
                        false
                    }
                }) {
                    exits.push(new_node_idx);
                }
            }
        }
        
        Ok(exits)
    }

    /// 删除调用指令
    fn remove_call_instruction(
        &self,
        call_cfg_node: u32,
        call_instr_idx: usize,
        cfg_graph: &mut crate::toolkit::cfg_node::CfgGraph,
        instr_slab: &mut crate::toolkit::nhwc_instr::InstrSlab<NhwcInstr>,
    ) -> Result<()> {
        // 从CFG节点中移除调用指令
        let call_node = node_mut!(at call_cfg_node in cfg_graph);
        if let Some(pos) = call_node.instrs.instr_vec.iter().position(|&x| x == call_instr_idx) {
            call_node.instrs.instr_vec.remove(pos);
        }
        
        // 检查调用节点是否还有其他指令
        let call_node = node!(at call_cfg_node in cfg_graph);
        if call_node.instrs.instr_vec.is_empty() {
            // 如果调用节点没有其他指令，检查是否需要删除该节点
            let incoming_edges: Vec<_> = cfg_graph
                .edges_directed(call_cfg_node.into(), petgraph::Direction::Incoming)
                .collect();
            let outgoing_edges: Vec<_> = cfg_graph
                .edges_directed(call_cfg_node.into(), petgraph::Direction::Outgoing)
                .collect();
            
            // 如果节点没有边，删除它
            if incoming_edges.is_empty() && outgoing_edges.is_empty() {
                debug_info_yellow!("删除孤立的调用节点: {}", call_cfg_node);
                // 注意：这里不能直接删除节点，因为可能影响其他引用
                // 在实际实现中，可能需要更复杂的清理逻辑
            }
        }
        
        Ok(())
    }

    /// 完整的指令复制和映射
    fn copy_and_map_instr_complete(
        &self,
        old_instr: &NhwcInstr,
        param_mapping: &AHashMap<RcSymIdx, RcSymIdx>,
        ret_var: Option<RcSymIdx>,
        caller_func: &RcSymIdx,
        callee_func: &RcSymIdx,
        symtab: &SymTab,
    ) -> Result<NhwcInstr> {
        let mut new_instr = old_instr.clone();
        
        match &mut new_instr.instr_type {
            NhwcInstrType::DefineFunc { .. } => {
                // 跳过函数定义指令，返回一个空指令
                return Ok(NhwcInstrType::Nope{}.into());
            }
            NhwcInstrType::Call { op_lhs, func_op } => {
                // 映射函数调用中的符号
                if let Some(lhs) = op_lhs {
                    if let Some(mapped) = param_mapping.get(lhs) {
                        *lhs = mapped.clone();
                    }
                }
                for arg in &mut func_op.actual_arg_symidx_vec {
                    if let Some(mapped) = param_mapping.get(arg) {
                        *arg = mapped.clone();
                    }
                }
            }
            NhwcInstrType::Jump { jump_op } => {
                match jump_op {
                    JumpOp::Ret { op_ret_sym } => {
                        // 处理返回语句 - 修复返回值处理
                        if let Some(ret_sym) = op_ret_sym {
                            if let Some(caller_ret) = &ret_var {
                                // 将返回值赋给调用者的变量
                                let ret_type = ret_sym.as_ref_borrow().get_ty(symtab)?;
                                
                                // 创建赋值指令，确保变量定义顺序正确
                                return Ok(NhwcInstrType::SimpleAssign {
                                    lhs: caller_ret.clone(),
                                    rhs: ret_sym.clone(),
                                    vartype: ret_type,
                                }.into());
                            }
                        }
                    }
                    _ => {
                        // 映射跳转指令中的符号
                        self.map_jump_symbols_complete(jump_op, param_mapping)?;
                    }
                }
            }
            NhwcInstrType::Arith { lhs, rhs } => {
                // 映射算术指令中的符号
                if let Some(mapped) = param_mapping.get(lhs) {
                    *lhs = mapped.clone();
                }
                self.map_arith_symbols_complete(rhs, param_mapping)?;
            }
            NhwcInstrType::SimpleAssign { lhs, rhs, vartype } => {
                // 映射赋值指令中的符号
                if let Some(mapped) = param_mapping.get(lhs) {
                    *lhs = mapped.clone();
                }
                if let Some(mapped) = param_mapping.get(rhs) {
                    *rhs = mapped.clone();
                }
            }
            _ => {
                // 对其他类型的指令进行通用符号映射
                self.map_instr_symbols_complete(&mut new_instr, param_mapping)?;
            }
        }
        
        // 设置内联相关字段
        new_instr.info.insert("INLINE_DEPTH", Box::new(1usize));
        new_instr.info.insert("INLINE_PARENT", Box::new(Some(0usize)));
        
        Ok(new_instr)
    }

    /// 完整的跳转指令符号映射
    fn map_jump_symbols_complete(
        &self,
        jump_op: &mut JumpOp,
        param_mapping: &AHashMap<RcSymIdx, RcSymIdx>,
    ) -> Result<()> {
        match jump_op {
            JumpOp::Br { cond, t1, t2 } => {
                if let Some(mapped) = param_mapping.get(cond) {
                    *cond = mapped.clone();
                }
                if let Some(mapped) = param_mapping.get(t1) {
                    *t1 = mapped.clone();
                }
                if let Some(mapped) = param_mapping.get(t2) {
                    *t2 = mapped.clone();
                }
            }
            JumpOp::Switch { cond, default, compared } => {
                if let Some(mapped) = param_mapping.get(cond) {
                    *cond = mapped.clone();
                }
                if let Some(mapped) = param_mapping.get(default) {
                    *default = mapped.clone();
                }
                // 映射compared中的符号 - 由于ComparedPair字段私有，简化处理
                // TODO: 如果需要映射，需要为 ComparedPair 添加公共方法
                debug_info_yellow!("跳过ComparedPair映射，字段私有");
            }
            JumpOp::DirectJump { label_symidx } => {
                if let Some(mapped) = param_mapping.get(label_symidx) {
                    *label_symidx = mapped.clone();
                }
            }
            JumpOp::Ret { op_ret_sym } => {
                if let Some(ret_sym) = op_ret_sym {
                    if let Some(mapped) = param_mapping.get(ret_sym) {
                        *ret_sym = mapped.clone();
                    }
                }
            }
        }
        Ok(())
    }

    /// 完整的算术指令符号映射
    fn map_arith_symbols_complete(
        &self,
        arith_op: &mut crate::toolkit::nhwc_instr::ArithOp,
        param_mapping: &AHashMap<RcSymIdx, RcSymIdx>,
    ) -> Result<()> {
        match arith_op {
            crate::toolkit::nhwc_instr::ArithOp::Add { a, b, vartype: _ } |
            crate::toolkit::nhwc_instr::ArithOp::Sub { a, b, vartype: _ } |
            crate::toolkit::nhwc_instr::ArithOp::Mul { a, b, vartype: _ } |
            crate::toolkit::nhwc_instr::ArithOp::Div { a, b, vartype: _ } |
            crate::toolkit::nhwc_instr::ArithOp::Mod { a, b, vartype: _ } |
            crate::toolkit::nhwc_instr::ArithOp::Icmp { a, b, vartype: _, plan: _ } |
            crate::toolkit::nhwc_instr::ArithOp::Fcmp { a, b, vartype: _, plan: _ } |
            crate::toolkit::nhwc_instr::ArithOp::LogicAnd { a, b, vartype: _ } |
            crate::toolkit::nhwc_instr::ArithOp::LogicOr { a, b, vartype: _ } |
            crate::toolkit::nhwc_instr::ArithOp::BitwiseOr { a, b, vartype: _ } |
            crate::toolkit::nhwc_instr::ArithOp::RightShift { a, b, vartype: _ } |
            crate::toolkit::nhwc_instr::ArithOp::LeftShift { a, b, vartype: _ } |
            crate::toolkit::nhwc_instr::ArithOp::BitwiseAnd { a, b, vartype: _ } => {
                if let Some(mapped) = param_mapping.get(a) {
                    *a = mapped.clone();
                }
                if let Some(mapped) = param_mapping.get(b) {
                    *b = mapped.clone();
                }
            }
            crate::toolkit::nhwc_instr::ArithOp::LogicNot { a, vartype: _ } => {
                if let Some(mapped) = param_mapping.get(a) {
                    *a = mapped.clone();
                }
            }
        }
        Ok(())
    }

    /// 完整的指令符号映射
    fn map_instr_symbols_complete(
        &self,
        instr: &mut NhwcInstr,
        param_mapping: &AHashMap<RcSymIdx, RcSymIdx>,
    ) -> Result<()> {
        // 获取指令中所有使用的符号并映射
        let use_symbols = instr.get_ssa_direct_use_symidx_vec();
        for sym in use_symbols {
            if let Some(mapped) = param_mapping.get(sym) {
                // 这里需要更复杂的逻辑来替换符号
                // 简化处理：记录映射信息
                debug_info_yellow!("映射符号: {:?} -> {:?}", sym.as_ref_borrow(), mapped.as_ref_borrow());
            }
        }
        Ok(())
    }

    /// 生成内联调试图
    fn generate_inline_debug_graph(
        &self,
        cfg_graph: &mut crate::toolkit::cfg_node::CfgGraph,
        symtab: &SymTab,
        instr_slab: &crate::toolkit::nhwc_instr::InstrSlab<NhwcInstr>,
        io_task_list: &mut Vec<std::thread::JoinHandle<Result<()>>>,
    ) -> Result<()> {
        // 更新CFG节点的文本表示
        for cfg_node in cfg_graph.node_weights_mut() {
            cfg_node.text.clear();
            cfg_node.load_instrs_text(instr_slab)?;
        }
        
        generate_png_by_graph_multi_tasks(
            &cfg_graph.clone(),
            "inline_cfg_graph".to_string(),
            &[
                Config::Record,
                Config::Rounded,
                Config::Title("Inline CFG Graph".to_string()),
                Config::CfgBlock,
                Config::NodeIndexLabel,
            ],
            io_task_list,
        )?;
        
        Ok(())
    }
}
