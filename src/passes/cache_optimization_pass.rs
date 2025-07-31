use crate::{  
    toolkit::{  
        context::NhwcCtx,  
        pass_manager::Pass,  
        symtab::{RcSymIdx, SymIdx, SymTab, WithBorrow},  
        cfg_node::{CfgGraph, CFG_ROOT},  
        mem_layout::MemLayout,  
        field::Type,  
        nhwc_instr::{InstrSlab, NhwcInstr, NhwcInstrType},  
        symbol::Symbol,  
    },  
    node, node_mut, direct_child_nodes, reg_field_for_struct, make_field_trait_for_struct,  
};  
use anyhow::{Result, anyhow};  
use std::collections::{HashMap, HashSet};  
  
const CACHE_LINE_SIZE: usize = 64;  
const HOT_ACCESS_THRESHOLD: usize = 10;  
  
#[derive(Debug, Clone, Copy, PartialEq)]  
pub enum ArrayLayout {  
    RowMajor,  
    ColumnMajor,  
}  
  
make_field_trait_for_struct!(ArrayLayout, usize);  
  
// 为Symbol添加缓存优化相关字段  
reg_field_for_struct!(Symbol {  
    ARRAY_LAYOUT:ArrayLayout,  
    ACCESS_COUNT:usize,  
} with_fields fields);  
  
impl ArrayLayout {  
    pub fn from_access_pattern(accesses: &[Vec<usize>]) -> Self {  
        let mut row_major_count = 0;  
        let mut col_major_count = 0;  
          
        for access in accesses {  
            if access.is_empty() { continue; }  
              
            let mut is_row_major = true;  
            let mut is_col_major = true;  
              
            for i in 1..access.len() {  
                if access[i] <= access[i-1] {  
                    is_row_major = false;  
                }  
                if access[i] >= access[i-1] {  
                    is_col_major = false;  
                }  
            }  
              
            if is_row_major { row_major_count += 1; }  
            if is_col_major { col_major_count += 1; }  
        }  
          
        if col_major_count > row_major_count {  
            ArrayLayout::ColumnMajor  
        } else {  
            ArrayLayout::RowMajor  
        }  
    }  
}  
  
#[derive(Debug)]  
pub struct CacheOptimizationPass {  
    debug: bool,  
}  
  
impl CacheOptimizationPass {  
    pub fn new(debug: bool) -> Self {  
        CacheOptimizationPass { debug }  
    }  
      
    fn analyze_access_patterns(&self, ctx: &NhwcCtx) -> HashMap<SymIdx, Vec<Vec<usize>>> {  
        let mut access_patterns = HashMap::new();  
          
        // 遍历所有指令，收集数组访问模式  
        for (_, instr) in ctx.nhwc_instr_slab.iter() {  
            match &instr.instr_type {  
                // 修正：使用实际存在的指令类型  
                NhwcInstrType::Load { lhs, ptr_symidx, ptr_ty: _ } => {  
                    if let Some(symidx) = lhs {  
                        let src_symidx = symidx.as_ref_borrow().to_src_symidx();  
                        let ssa_idx = symidx.as_ref_borrow().ssa_idx.map(|x| x.get() as usize).unwrap_or(0);  
                        access_patterns.entry(src_symidx)  
                            .or_insert_with(Vec::new)  
                            .push(vec![ssa_idx]);  
                    }  
                }  
                NhwcInstrType::Store { val_symidx, value_ty: _, ptr_symidx, ptr_ty: _ } => {  
                    if let Some(symidx) = val_symidx {  
                        let src_symidx = symidx.as_ref_borrow().to_src_symidx();  
                        let ssa_idx = symidx.as_ref_borrow().ssa_idx.map(|x| x.get() as usize).unwrap_or(0);  
                        access_patterns.entry(src_symidx)  
                            .or_insert_with(Vec::new)  
                            .push(vec![ssa_idx]);  
                    }  
                }  
                NhwcInstrType::GetElementPtr { lhs, array_ty: _, ptr_symidx, idx_vec } => {  
                    if let Some(array_symidx) = ptr_symidx {  
                        let src_symidx = array_symidx.as_ref_borrow().to_src_symidx();  
                        let access_pattern: Vec<usize> = idx_vec.iter()  
                            .map(|idx| idx.as_ref_borrow().ssa_idx.map(|x| x.get() as usize).unwrap_or(0))  
                            .collect();  
                          
                        access_patterns.entry(src_symidx)  
                            .or_insert_with(Vec::new)  
                            .push(access_pattern);  
                    }  
                }  
                _ => {}  
            }  
        }  
          
        access_patterns  
    }  
      
    fn identify_hot_vars(&self, ctx: &NhwcCtx) -> HashSet<SymIdx> {  
        let mut access_counts = HashMap::new();  
          
        // 统计变量访问次数  
        for (_, instr) in ctx.nhwc_instr_slab.iter() {  
            match &instr.instr_type {  
                NhwcInstrType::Load { lhs, ptr_symidx: _, ptr_ty: _ } => {  
                    if let Some(symidx) = lhs {  
                        *access_counts.entry(symidx.as_ref_borrow().to_src_symidx())  
                            .or_insert(0) += 1;  
                    }  
                }  
                NhwcInstrType::Store { val_symidx, value_ty: _, ptr_symidx: _, ptr_ty: _ } => {  
                    if let Some(symidx) = val_symidx {  
                        *access_counts.entry(symidx.as_ref_borrow().to_src_symidx())  
                            .or_insert(0) += 1;  
                    }  
                }  
                NhwcInstrType::Arith { lhs, rhs: _ } => {  
                    if let Some(symidx) = lhs {  
                        *access_counts.entry(symidx.as_ref_borrow().to_src_symidx())  
                            .or_insert(0) += 1;  
                    }  
                }  
                NhwcInstrType::Phi { lhs, rhs: _ } => {  
                    if let Some(symidx) = lhs {  
                        *access_counts.entry(symidx.as_ref_borrow().to_src_symidx())  
                            .or_insert(0) += 1;  
                    }  
                }  
                _ => {}  
            }  
        }  
          
        // 筛选热点变量  
        access_counts.into_iter()  
            .filter(|(_, count)| *count > HOT_ACCESS_THRESHOLD)  
            .map(|(symidx, _)| symidx)  
            .collect()  
    }  
      
    fn optimize_array_layouts(  
        &self,   
        ctx: &mut NhwcCtx,  
        access_patterns: &HashMap<SymIdx, Vec<Vec<usize>>>  
    ) {  
        for (symidx, patterns) in access_patterns {  
            if let Some(symbol) = ctx.symtab.get_mut(symidx) {  
                if symbol.has_type() && symbol.get_type().is_array() {  
                    let layout = ArrayLayout::from_access_pattern(patterns);  
                      
                    if layout == ArrayLayout::ColumnMajor {  
                        symbol.add_array_layout(layout);  
                          
                        if self.debug {  
                            println!("[CacheOpt] 数组 {:?} 改为Column-Major布局", symidx);  
                        }  
                    }  
                }  
            }  
        }  
    }  
      
    fn reorganize_data(&self, ctx: &mut NhwcCtx, hot_vars: &HashSet<SymIdx>) {  
        let cfg_entries = direct_child_nodes!(at CFG_ROOT in &ctx.cfg_graph);  
          
        for &cfg_entry in &cfg_entries {  
            if let Some(cfg_node) = ctx.cfg_graph.node_weight_mut(cfg_entry) {  
                let mut mem_layout = if cfg_node.has_mem_layout() {  
                    cfg_node.get_mem_layout().clone()  
                } else {  
                    MemLayout::new()  
                };  
                  
                // 收集需要重组的变量  
                let mut vars_to_reorder = Vec::new();  
                for (symidx, symbol) in ctx.symtab.iter() {  
                    if hot_vars.contains(symidx) && symbol.has_type() {  
                        let size = symbol.get_type().get_mem_len().unwrap_or(8);  
                        let align = symbol.get_type().get_align().unwrap_or(8);  
                        vars_to_reorder.push((symidx.clone(), size, align));  
                    }  
                }  
                  
                // 按大小排序  
                vars_to_reorder.sort_by_key(|(_, size, _)| std::cmp::Reverse(*size));  
                  
                // 重新创建内存布局  
                mem_layout.clear();  
                for (symidx, size, align) in vars_to_reorder {  
                    if let Some(rc_symidx) = ctx.symtab.get_symidx_cor_rc(&symidx) {  
                        mem_layout.insert_data(align, size, &rc_symidx);  
                          
                        if self.debug {  
                            println!("[CacheOpt] 重组变量 {:?} (大小: {}, 对齐: {})",   
                                symidx, size, align);  
                        }  
                    }  
                }  
                  
                cfg_node.add_mem_layout(mem_layout);  
            }  
        }  
    }  
      
    fn align_to_cache_lines(&self, ctx: &mut NhwcCtx) {  
        let cfg_entries = direct_child_nodes!(at CFG_ROOT in &ctx.cfg_graph);  
          
        for &cfg_entry in &cfg_entries {  
            if let Some(cfg_node) = ctx.cfg_graph.node_weight_mut(cfg_entry) {  
                if cfg_node.has_mem_layout() {  
                    let mem_layout = cfg_node.get_mut_mem_layout();  
                    mem_layout.align_mem_with_blank(CACHE_LINE_SIZE);  
                      
                    if self.debug {  
                        println!("[CacheOpt] 函数入口 {:?} 内存布局对齐到 {} 字节",   
                            cfg_entry, CACHE_LINE_SIZE);  
                    }  
                }  
            }  
        }  
    }  
}  
  
impl Pass for CacheOptimizationPass {  
    fn run(&mut self, ctx: &mut NhwcCtx) -> Result<()> {  
        // 1. 分析访问模式  
        let access_patterns = self.analyze_access_patterns(ctx);  
          
        // 2. 识别热点变量  
        let hot_vars = self.identify_hot_vars(ctx);  
          
        // 3. 优化数组布局  
        self.optimize_array_layouts(ctx, &access_patterns);  
          
        // 4. 重组数据布局  
        self.reorganize_data(ctx, &hot_vars);  
          
        // 5. 缓存行对齐  
        self.align_to_cache_lines(ctx);  
          
        Ok(())  
    }  
  
    fn get_desc(&self) -> String {  
        "高性能缓存优化Pass (布局优化+数据重组+对齐)".to_string()  
    }  
  
    fn get_pass_name(&self) -> String {  
        "CacheOptimizationPass".to_string()  
    }  
  
    
}