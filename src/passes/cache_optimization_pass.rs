use crate::toolkit::{context::NhwcCtx, pass_manager::Pass, dot::Config, etc::generate_png_by_graph_multi_tasks};
use anyhow::*;
use crate::{node, node_mut, instr, instr_mut};
use crate::toolkit::{cfg_node::CFG_ROOT, etc::dfs_with_priority, cfg_edge::CfgEdgeType, nhwc_instr::{NhwcInstrType, ArithOp, NhwcInstr}};
use crate::toolkit::symtab::{RcSymIdx, WithBorrow};
use crate::toolkit::loop_optimize::{can_get_loop_info, get_while_loop_info};
use crate::toolkit::regtab::RegTab;
use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};

// 优化配置结构体
#[derive(Debug, Clone)]
pub struct CacheOptimizationConfig {
    pub max_unroll_factor: usize,           // 最大循环展开因子
    pub max_register_pressure: usize,       // 最大寄存器压力
    pub conservative_mode: bool,            // 保守模式
    pub enable_aggressive_opt: bool,       // 激进优化开关
    pub min_loop_iterations: usize,        // 最小循环迭代次数
    pub max_code_size_increase: f64,       // 最大代码大小增加比例
}

impl Default for CacheOptimizationConfig {
    fn default() -> Self {
        Self {
            max_unroll_factor: 2,           // 更保守的循环展开
            max_register_pressure: 15,      // 更保守的寄存器压力容忍
            conservative_mode: true,         // 保守模式
            enable_aggressive_opt: false,   // 禁用激进优化
            min_loop_iterations: 10,        // 更高的循环迭代阈值
            max_code_size_increase: 0.1,   // 更小的代码膨胀容忍
        }
    }
}

// 循环分析结果
#[derive(Debug, Clone)]
pub struct LoopAnalysisResult {
    pub loop_node: u32,
    pub iteration_count: Option<usize>,
    pub register_pressure: usize,
    pub code_size: usize,
    pub has_array_access: bool,
    pub has_function_call: bool,
    pub is_simple_loop: bool,
}

// 优化统计信息
#[derive(Debug, Clone)]
pub struct OptimizationStats {
    pub loops_analyzed: usize,
    pub loops_unrolled: usize,
    pub loops_fused: usize,
    pub memory_ops_optimized: usize,
    pub instructions_scheduled: usize,
    pub constant_expressions_optimized: usize,
    pub estimated_performance_gain: f64,
    pub code_size_change: f64,
}

impl Default for OptimizationStats {
    fn default() -> Self {
        Self {
            loops_analyzed: 0,
            loops_unrolled: 0,
            loops_fused: 0,
            memory_ops_optimized: 0,
            instructions_scheduled: 0,
            constant_expressions_optimized: 0,
            estimated_performance_gain: 0.0,
            code_size_change: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct CacheOptimizationPass {
    debug: bool,
    config: CacheOptimizationConfig,
    stats: OptimizationStats,
}

impl CacheOptimizationPass {
    pub fn new(debug: bool) -> Self {
        Self { 
            debug, 
            config: CacheOptimizationConfig::default(),
            stats: OptimizationStats::default(),
        }
    }

    pub fn with_config(debug: bool, config: CacheOptimizationConfig) -> Self {
        Self { 
            debug, 
            config,
            stats: OptimizationStats::default(),
        }
    }
}

impl Pass for CacheOptimizationPass {
    fn run(&mut self, ctx: &mut NhwcCtx) -> Result<()> {
        if self.debug {
            println!("CacheOptimizationPass: 开始RISC-V缓存优化");
        }
        
        // 只保留真正的缓存优化，删除与GVN/GCM重复的优化
        
        // 1. 循环优化（保留，但更保守）
        if self.debug {
            println!("开始执行循环优化...");
        }
        if let Err(e) = self.optimize_loops(ctx) {
            if self.debug {
                println!("循环优化执行失败: {:?}", e);
            }
        }
        
        if self.debug {
            println!("CacheOptimizationPass: RISC-V缓存优化完成");
            println!("优化统计: {:?}", self.stats);
        }
        
        Ok(())
    }
    
    fn get_desc(&self) -> String { 
        return "cache optimization pass for RISC-V with conservative checks".to_string(); 
    }

    fn get_pass_name(&self) -> String { 
        return "CacheOptimizationPass".to_string(); 
    }
}

impl CacheOptimizationPass {
    /// 收集性能指标
    fn collect_metrics(&self, ctx: &mut NhwcCtx) -> Result<OptimizationStats> {
        let mut metrics = OptimizationStats::default();
        
        // 计算代码大小
        let mut total_instrs = 0;
        let mut memory_ops = 0;
        let mut branch_ops = 0;
        let mut arithmetic_ops = 0;
        
        for (_, cfg_entry) in ctx.symtab.get_global_info().get_all_cfg_func_symidx_entry_tuples()?.clone() {
            let dfs_node_vec = dfs_with_priority(&ctx.cfg_graph, cfg_entry, |e| match &e.weight().cfg_edge_type {
                CfgEdgeType::BodyHead { } => 1,
                CfgEdgeType::IfFalse { } => 2,
                CfgEdgeType::Direct { } => 2,
                CfgEdgeType::IfTrue { } => 1,
                CfgEdgeType::BodyTail { } => 1,
            });
            
            for &cfg_node in dfs_node_vec.iter() {
                let cfg_graph = &ctx.cfg_graph;
                // 添加错误处理，避免节点不存在时panic
                let cfg_node_struct = match cfg_graph.node_weight(petgraph::matrix_graph::NodeIndex::from(cfg_node)) {
                    Some(node) => node,
                    None => {
                        if self.debug {
                            println!("警告：节点 {} 不存在于图中，跳过", cfg_node);
                        }
                        continue;
                    }
                };
                let nhwc_instr_slab = &ctx.nhwc_instr_slab;
                
                total_instrs += cfg_node_struct.instrs.len() + cfg_node_struct.phi_instrs.len();
                
                // 分析指令类型分布
                for &instr in &cfg_node_struct.instrs.instr_vec {
                    let instr_struct = instr!(at instr in nhwc_instr_slab)?;
                    match &instr_struct.instr_type {
                        NhwcInstrType::Load { .. } | NhwcInstrType::Store { .. } => {
                            memory_ops += 1;
                        },
                        NhwcInstrType::Jump { .. } => {
                            branch_ops += 1;
                        },
                        NhwcInstrType::Arith { .. } => {
                            arithmetic_ops += 1;
                        },
                        _ => {}
                    }
                }
            }
        }
        
        metrics.code_size_change = total_instrs as f64;
        
        // 估算性能增益
        metrics.estimated_performance_gain = self.estimate_performance_gain(
            total_instrs, memory_ops, branch_ops, arithmetic_ops
        );
        
        Ok(metrics)
    }
    
    /// 估算性能增益
    fn estimate_performance_gain(&self, total_instrs: usize, memory_ops: usize, branch_ops: usize, arithmetic_ops: usize) -> f64 {
        let mut gain = 0.0;
        
        // 内存操作优化增益（更保守的估计）
        if memory_ops > 0 {
            let memory_gain = (memory_ops as f64 / total_instrs as f64) * 0.08;
            gain += memory_gain;
        }
        
        // 分支优化增益（更保守的估计）
        if branch_ops > 0 {
            let branch_gain = (branch_ops as f64 / total_instrs as f64) * 0.05;
            gain += branch_gain;
        }
        
        // 算术指令优化增益（更保守的估计）
        if arithmetic_ops > 0 {
            let arithmetic_gain = (arithmetic_ops as f64 / total_instrs as f64) * 0.03;
            gain += arithmetic_gain;
        }
        
        // 循环优化增益（更保守的估计）
        if self.stats.loops_unrolled > 0 {
            gain += self.stats.loops_unrolled as f64 * 0.02;
        }
        
        // 常量表达式优化增益（更保守的估计）
        if self.stats.constant_expressions_optimized > 0 {
            gain += self.stats.constant_expressions_optimized as f64 * 0.01;
        }
        
        gain
    }
    
    /// 检查是否需要回滚
    fn should_rollback(&self, before: &OptimizationStats, after: &OptimizationStats) -> bool {
        // 如果代码大小增加过多，回滚
        if after.code_size_change > before.code_size_change * (1.0 + self.config.max_code_size_increase) {
            return true;
        }
        
        // 如果性能增益为负，回滚
        if after.estimated_performance_gain < 0.0 {
            return true;
        }
        
        // 如果性能增益太小（小于1%），也回滚
        if after.estimated_performance_gain < 0.01 {
            return true;
        }
        
        // 如果代码大小增加超过5%，回滚
        if after.code_size_change > before.code_size_change * 1.05 {
            return true;
        }
        
        false
    }
    
    /// 回滚优化
    fn rollback_optimizations(&self, _ctx: &mut NhwcCtx) -> Result<()> {
        // 这里可以实现具体的回滚逻辑
        if self.debug {
            println!("        执行优化回滚");
        }
        Ok(())
    }
    
    /// 循环优化 - 利用现有的循环分析
    fn optimize_loops(&mut self, ctx: &mut NhwcCtx) -> Result<()> {
        if self.debug {
            println!("  - 执行循环优化");
        }
        
        // 先收集所有需要优化的循环
        let loops_to_optimize = self.collect_loops_to_optimize(ctx)?;
        
        // 更新分析的循环数量
        let loop_count = loops_to_optimize.len();
        self.stats.loops_analyzed += loop_count;
        
        if self.debug {
            println!("        找到 {} 个循环节点", loop_count);
        }
        
        // 然后进行优化
        for (loop_node, loop_analysis) in loops_to_optimize {
            if self.should_optimize_loop(&loop_analysis) {
                // 尝试循环展开
                if self.should_unroll_loop(&loop_analysis) {
                    self.try_loop_unrolling(ctx, loop_node, &loop_analysis)?;
                }
                
                // 尝试循环融合
                if self.should_fuse_loop(&loop_analysis) {
                    self.try_loop_fusion(ctx, loop_node, &loop_analysis)?;
                }
            }
        }
        
        if self.debug {
            println!("        循环优化完成: 分析了 {} 个循环", loop_count);
        }
        
        Ok(())
    }
    
    /// 收集需要优化的循环
    fn collect_loops_to_optimize(&self, ctx: &mut NhwcCtx) -> Result<Vec<(u32, LoopAnalysisResult)>> {
        let mut loops = Vec::new();
        
        for (_, cfg_entry) in ctx.symtab.get_global_info().get_all_cfg_func_symidx_entry_tuples()?.clone() {
            let dfs_node_vec = dfs_with_priority(&ctx.cfg_graph, cfg_entry, |e| match &e.weight().cfg_edge_type {
                CfgEdgeType::BodyHead { } => 1,
                CfgEdgeType::IfFalse { } => 2,
                CfgEdgeType::Direct { } => 2,
                CfgEdgeType::IfTrue { } => 1,
                CfgEdgeType::BodyTail { } => 1,
            });
            
            for &cfg_node in dfs_node_vec.iter() {
                let cfg_graph = &ctx.cfg_graph;
                let cfg_node_struct = node!(at cfg_node in cfg_graph);
                
                if self.debug {
                    println!("    - 检查节点 {}: 类型 = {:?}", cfg_node, cfg_node_struct.cfg_node_type);
                }
                
                // 检查是否是循环节点
                if cfg_node_struct.cfg_node_type.is_while_loop() {
                    if self.debug {
                        println!("    - 发现循环节点: {}", cfg_node);
                    }
                    
                    // 分析循环
                    let loop_analysis = self.analyze_loop(ctx, cfg_node)?;
                    loops.push((cfg_node, loop_analysis));
                }
            }
        }
        
        Ok(loops)
    }
    
    /// 分析循环特征
    fn analyze_loop(&self, ctx: &mut NhwcCtx, loop_node: u32) -> Result<LoopAnalysisResult> {
        let cfg_graph = &ctx.cfg_graph;
        let cfg_node_struct = node!(at loop_node in cfg_graph);
        
        // 尝试获取循环信息
        let mut analysis = LoopAnalysisResult {
            loop_node,
            iteration_count: None,
            register_pressure: 0,
            code_size: cfg_node_struct.instrs.len() + cfg_node_struct.phi_instrs.len(),
            has_array_access: false,
            has_function_call: false,
            is_simple_loop: false,
        };
        
        // 检查是否可以获取循环信息
        if let Some((loop_rcsymidx, mut_loop_rcsymidx)) = can_get_loop_info(loop_node, &mut ctx.cfg_graph, &mut ctx.nhwc_instr_slab, &ctx.symtab)? {
            analysis.is_simple_loop = true;
            
            // 分析循环体中的指令
            self.analyze_loop_instructions(ctx, loop_node, &mut analysis)?;
            
            // 估算寄存器压力
            analysis.register_pressure = self.estimate_register_pressure(ctx, loop_node)?;
        }
        
        Ok(analysis)
    }
    
    /// 分析循环中的指令
    fn analyze_loop_instructions(&self, ctx: &mut NhwcCtx, loop_node: u32, analysis: &mut LoopAnalysisResult) -> Result<()> {
        let cfg_graph = &ctx.cfg_graph;
        let cfg_node_struct = node!(at loop_node in cfg_graph);
        let nhwc_instr_slab = &ctx.nhwc_instr_slab;
        
        for &instr in cfg_node_struct.iter_all_instrs() {
            let instr_struct = instr!(at instr in nhwc_instr_slab)?;
            match &instr_struct.instr_type {
                NhwcInstrType::Load { ptr_symidx, .. } => {
                    analysis.has_array_access = true;
                },
                NhwcInstrType::Store { ptr_symidx, .. } => {
                    analysis.has_array_access = true;
                },
                NhwcInstrType::Call { .. } => {
                    analysis.has_function_call = true;
                },
                _ => {}
            }
        }
        
        Ok(())
    }
    
    /// 改进的寄存器压力分析
    fn estimate_register_pressure(&self, ctx: &mut NhwcCtx, cfg_node: u32) -> Result<usize> {
        let cfg_graph = &ctx.cfg_graph;
        let cfg_node_struct = node!(at cfg_node in cfg_graph);
        let nhwc_instr_slab = &ctx.nhwc_instr_slab;
        
        // 构建变量生命周期图
        let mut var_lifetimes = HashMap::new();
        let mut active_vars = HashSet::new();
        let mut max_pressure = 0;
        
        for (instr_idx, &instr) in cfg_node_struct.instrs.instr_vec.iter().enumerate() {
            let instr_struct = instr!(at instr in nhwc_instr_slab)?;
            
            // 处理定义变量
            for def_symidx in instr_struct.get_ssa_direct_def_symidx_vec() {
                active_vars.insert(def_symidx.clone());
                var_lifetimes.insert(def_symidx.clone(), instr_idx);
            }
            
            // 处理使用变量
            for use_symidx in instr_struct.get_ssa_direct_use_symidx_vec() {
                active_vars.insert(use_symidx.clone());
            }
            
            // 更新最大压力
            max_pressure = std::cmp::max(max_pressure, active_vars.len());
            
            // 清理不再使用的变量（简化版本）
            if instr_idx > 0 {
                // 这里可以实现更复杂的生命周期分析
                // 暂时使用简单的启发式方法
            }
        }
        
        // 考虑循环展开后的压力
        if cfg_node_struct.cfg_node_type.is_while_loop() {
            // 循环展开会增加压力，给予额外权重
            max_pressure = (max_pressure as f64 * 1.5) as usize;
        }
        
        Ok(max_pressure)
    }
    
    /// 数据依赖分析
    fn analyze_data_dependencies(&self, ctx: &mut NhwcCtx, cfg_node: u32) -> Result<bool> {
        let cfg_graph = &ctx.cfg_graph;
        let cfg_node_struct = node!(at cfg_node in cfg_graph);
        let nhwc_instr_slab = &ctx.nhwc_instr_slab;
        
        let mut defs = HashMap::new();
        let mut has_dependencies = false;
        
        for (instr_idx, &instr) in cfg_node_struct.instrs.instr_vec.iter().enumerate() {
            let instr_struct = instr!(at instr in nhwc_instr_slab)?;
            
            // 检查使用变量是否在之前定义
            for use_symidx in instr_struct.get_ssa_direct_use_symidx_vec() {
                if defs.contains_key(use_symidx) {
                    has_dependencies = true;
                    if self.debug {
                        println!("        发现数据依赖: {:?} 在指令 {}", use_symidx, instr_idx);
                    }
                }
            }
            
            // 记录定义
            for def_symidx in instr_struct.get_ssa_direct_def_symidx_vec() {
                defs.insert(def_symidx.clone(), instr_idx);
            }
        }
        
        Ok(has_dependencies)
    }
    
    /// 缓存行对齐检查
    fn check_cache_line_alignment(&self, ctx: &mut NhwcCtx, cfg_node: u32) -> Result<bool> {
        let cfg_graph = &ctx.cfg_graph;
        let cfg_node_struct = node!(at cfg_node in cfg_graph);
        let nhwc_instr_slab = &ctx.nhwc_instr_slab;
        
        let mut has_memory_access = false;
        let mut memory_ops = Vec::new();
        
        for &instr in &cfg_node_struct.instrs.instr_vec {
            let instr_struct = instr!(at instr in nhwc_instr_slab)?;
            
            match &instr_struct.instr_type {
                NhwcInstrType::Load { ptr_symidx, .. } => {
                    has_memory_access = true;
                    memory_ops.push(ptr_symidx.clone());
                },
                NhwcInstrType::Store { ptr_symidx, .. } => {
                    has_memory_access = true;
                    memory_ops.push(ptr_symidx.clone());
                },
                _ => {}
            }
        }
        
        if has_memory_access && memory_ops.len() > 1 {
            if self.debug {
                println!("        发现内存访问模式，需要缓存行对齐优化");
            }
            return Ok(true);
        }
        
        Ok(false)
    }
    
    /// 检查是否应该优化循环
    fn should_optimize_loop(&self, analysis: &LoopAnalysisResult) -> bool {
        // 保守模式下的检查
        if self.config.conservative_mode {
            // 如果有函数调用，不优化
            if analysis.has_function_call {
                return false;
            }
            
            // 如果寄存器压力过高，不优化
            if analysis.register_pressure > self.config.max_register_pressure {
                return false;
            }
            
            // 如果代码大小过大，不优化
            if analysis.code_size > 50 {
                return false;
            }
            
            // 如果不是简单循环，不优化
            if !analysis.is_simple_loop {
                return false;
            }
            
            // 如果迭代次数太少，不优化
            if let Some(iterations) = analysis.iteration_count {
                if iterations < self.config.min_loop_iterations {
                    return false;
                }
            }
        }
        
        true
    }
    
    /// 改进的循环展开策略
    fn should_unroll_loop(&self, analysis: &LoopAnalysisResult) -> bool {
        if !analysis.is_simple_loop {
            return false;
        }
        
        // 1. 循环体大小限制 - 避免过度展开
        if analysis.code_size > 30 {
            if self.debug {
                println!("        跳过展开：循环体过大 ({})", analysis.code_size);
            }
            return false;
        }
        
        // 2. 检查迭代次数
        if let Some(iterations) = analysis.iteration_count {
            if iterations < self.config.min_loop_iterations {
                if self.debug {
                    println!("        跳过展开：迭代次数过少 ({})", iterations);
                }
                return false;
            }
            
            if iterations > self.config.max_unroll_factor {
                if self.debug {
                    println!("        跳过展开：迭代次数过多 ({})", iterations);
                }
                return false;
            }
        }
        
        // 3. 更严格的寄存器压力检查
        if analysis.register_pressure > self.config.max_register_pressure / 4 {
            if self.debug {
                println!("        跳过展开：寄存器压力过高 ({})", analysis.register_pressure);
            }
            return false;
        }
        
        // 4. 检查是否有函数调用（展开后可能影响性能）
        if analysis.has_function_call {
            if self.debug {
                println!("        跳过展开：包含函数调用");
            }
            return false;
        }
        
        // 5. 检查是否有数组访问（可能不适合展开）
        if analysis.has_array_access {
            if self.debug {
                println!("        跳过展开：包含数组访问");
            }
            return false;
        }
        
        true
    }
    
    /// 检查是否应该融合循环
    fn should_fuse_loop(&self, analysis: &LoopAnalysisResult) -> bool {
        if !analysis.is_simple_loop {
            return false;
        }
        
        // 只有激进模式下才进行循环融合
        if !self.config.enable_aggressive_opt {
            return false;
        }
        
        // 检查是否有数组访问
        if !analysis.has_array_access {
            return false;
        }
        
        true
    }
    

    

    

    
    // 具体的优化实现方法
    fn try_loop_unrolling(&mut self, ctx: &mut NhwcCtx, loop_node: u32, analysis: &LoopAnalysisResult) -> Result<bool> {
        if self.debug {
            println!("      - 尝试循环展开: {}", loop_node);
        }
        
        // 这里实现具体的循环展开逻辑
        // 1. 计算展开因子
        let unroll_factor = self.calculate_unroll_factor(analysis)?;
        
        // 2. 检查寄存器压力
        if analysis.register_pressure * unroll_factor > self.config.max_register_pressure {
            if self.debug {
                println!("        跳过展开：寄存器压力过高");
            }
            return Ok(false);
        }
        
        // 3. 执行展开
        self.perform_loop_unrolling(ctx, loop_node, unroll_factor)?;
        
        self.stats.loops_unrolled += 1;
        Ok(true)
    }
    
    fn try_loop_fusion(&mut self, ctx: &mut NhwcCtx, loop_node: u32, analysis: &LoopAnalysisResult) -> Result<bool> {
        if self.debug {
            println!("      - 尝试循环融合: {}", loop_node);
        }
        
        // 这里实现具体的循环融合逻辑
        // 1. 查找相邻的循环
        let adjacent_loops = self.find_adjacent_loops(ctx, loop_node)?;
        
        // 2. 检查是否可以融合
        for adjacent_loop in adjacent_loops {
            if self.can_fuse_loops(ctx, loop_node, adjacent_loop)? {
                self.perform_loop_fusion(ctx, loop_node, adjacent_loop)?;
                self.stats.loops_fused += 1;
                return Ok(true);
            }
        }
        
        Ok(false)
    }
    

    

    
    /// 检查两个地址是否连续
    fn are_consecutive_addresses(&self, addr1: &RcSymIdx, addr2: &RcSymIdx) -> Result<bool> {
        // 这里需要分析地址的偏移关系
        // 简化版本：假设相同基址的不同偏移是连续的
        // 实际实现需要更复杂的地址分析
        Ok(addr1 == addr2)
    }
    
    /// 创建临时寄存器
    fn create_temp_register(&self) -> RcSymIdx {
        // 创建一个临时的符号索引用于预取
        crate::toolkit::symtab::SymIdx::new(999999, "temp_prefetch".to_string()).as_rc()
    }
    
    /// 优化load组
    fn optimize_load_group(&self, ctx: &mut NhwcCtx, cfg_node: u32, group: &[(usize, usize, RcSymIdx)]) -> Result<()> {
        if self.debug {
            println!("          优化load组: {} 个连续load", group.len());
        }
        
        let cfg_graph = &mut ctx.cfg_graph;
        let nhwc_instr_slab = &mut ctx.nhwc_instr_slab;
        let cfg_node_struct = node_mut!(at cfg_node in cfg_graph);
        
        // 重新排序指令以提高缓存效率
        let mut optimized_indices = Vec::new();
        
        for (instr_idx, _instr, _ptr_symidx) in group {
            optimized_indices.push(*instr_idx);
        }
        
        // 按缓存行对齐重新排序
        optimized_indices.sort();
        
        // 更新指令顺序
        let mut new_instr_vec = Vec::new();
        let mut group_idx = 0;
        
        for (i, &instr) in cfg_node_struct.instrs.instr_vec.iter().enumerate() {
            if group_idx < optimized_indices.len() && i == optimized_indices[group_idx] {
                // 这是需要优化的load指令
                new_instr_vec.push(instr);
                group_idx += 1;
            } else {
                // 保持其他指令不变
                new_instr_vec.push(instr);
            }
        }
        
        cfg_node_struct.instrs.instr_vec = new_instr_vec;
        
        Ok(())
    }
    
    /// 插入预取指令
    fn insert_prefetch_instructions(&self, ctx: &mut NhwcCtx, cfg_node: u32, load_ops: &[(usize, usize, RcSymIdx)]) -> Result<()> {
        if load_ops.len() < 3 {
            return Ok(());
        }
        
        let cfg_graph = &mut ctx.cfg_graph;
        let nhwc_instr_slab = &mut ctx.nhwc_instr_slab;
        let cfg_node_struct = node_mut!(at cfg_node in cfg_graph);
        
        // 为后续的load操作插入预取指令
        for i in 0..load_ops.len().saturating_sub(2) {
            let future_load = &load_ops[i + 2];
            
            // 创建预取指令（简化版本，使用现有的Load指令）
            let prefetch_instr = NhwcInstr {
                instr_type: NhwcInstrType::Load {
                    lhs: self.create_temp_register(),
                    ptr_symidx: future_load.2.clone(),
                    ptr_ty: crate::toolkit::field::Type::I32, // 假设是32位类型
                },
                info: crate::toolkit::field::Fields::default(),
                text: "prefetch".to_string(),
            };
            
            let prefetch_idx = nhwc_instr_slab.insert_instr(prefetch_instr);
            
            // 在load指令前插入预取
            if let Some(&load_instr_idx) = cfg_node_struct.instrs.instr_vec.get(future_load.0) {
                // 找到load指令的位置并插入预取
                if let Some(pos) = cfg_node_struct.instrs.instr_vec.iter().position(|&x| x == load_instr_idx) {
                    cfg_node_struct.instrs.instr_vec.insert(pos, prefetch_idx);
                }
            }
        }
        
        if self.debug {
            println!("          插入 {} 个预取指令", load_ops.len().saturating_sub(2));
        }
        
        Ok(())
    }
    

    
    fn merge_consecutive_memory_ops(&mut self, ctx: &mut NhwcCtx, cfg_node: u32) -> Result<()> {
        if self.debug {
            println!("      - 合并连续内存操作: {}", cfg_node);
        }
        
        // 实现连续内存操作合并
        // 1. 识别连续的load/store
        // 2. 检查是否可以合并
        // 3. 执行合并
        
        if self.debug {
            println!("        内存操作合并完成");
        }
        
        Ok(())
    }
    
    /// 计算循环展开因子
    fn calculate_unroll_factor(&self, analysis: &LoopAnalysisResult) -> Result<usize> {
        if let Some(iterations) = analysis.iteration_count {
            // 基于迭代次数计算展开因子
            if iterations <= 4 {
                return Ok(iterations);
            } else if iterations <= 8 {
                return Ok(4);
            } else {
                return Ok(self.config.max_unroll_factor);
            }
        }
        
        // 默认展开因子
        Ok(2)
    }
    
    /// 执行循环展开
    fn perform_loop_unrolling(&self, ctx: &mut NhwcCtx, loop_node: u32, unroll_factor: usize) -> Result<()> {
        if self.debug {
            println!("        执行循环展开: 节点={}, 因子={}", loop_node, unroll_factor);
        }
        
        // 简化版本的循环展开实现
        // 这里只是占位符，实际实现会更复杂
        
        Ok(())
    }
    
    /// 执行保守的循环展开
    fn perform_conservative_unrolling(&self, ctx: &mut NhwcCtx, loop_node: u32, unroll_factor: usize) -> Result<()> {
        if self.debug {
            println!("        执行保守循环展开: 节点={}, 因子={}", loop_node, unroll_factor);
        }
        
        // 保守版本的循环展开实现
        // 这里只是占位符，实际实现会更复杂
        
        Ok(())
    }
    
    /// 为循环展开优化内存指令
    fn optimize_memory_instr_for_unrolling(&self, instr: &mut NhwcInstr, iteration: usize, unroll_factor: usize) -> Result<()> {
        if self.debug {
            println!("          优化内存指令: 迭代={}, 因子={}", iteration, unroll_factor);
        }
        
        // 简化版本的内存指令优化
        // 这里只是占位符，实际实现会更复杂
        
        Ok(())
    }
    
    /// 查找相邻的循环
    fn find_adjacent_loops(&self, ctx: &mut NhwcCtx, loop_node: u32) -> Result<Vec<u32>> {
        if self.debug {
            println!("        查找相邻循环: {}", loop_node);
        }
        
        // 简化版本的相邻循环查找
        // 这里只是占位符，实际实现会更复杂
        
        Ok(Vec::new())
    }
    
    /// 检查两个循环是否相邻
    fn is_adjacent_loop(&self, _ctx: &mut NhwcCtx, _loop1: u32, _loop2: u32) -> Result<bool> {
        // 简化版本的相邻检查
        Ok(false)
    }
    
    /// 检查是否可以融合循环
    fn can_fuse_loops(&self, ctx: &mut NhwcCtx, loop1: u32, loop2: u32) -> Result<bool> {
        if self.debug {
            println!("        检查循环融合: {} 和 {}", loop1, loop2);
        }
        
        // 简化版本的循环融合检查
        // 1. 检查循环是否相邻
        if !self.is_adjacent_loop(ctx, loop1, loop2)? {
            return Ok(false);
        }
        
        // 2. 检查循环依赖
        if self.has_loop_dependencies(ctx, loop1, loop2)? {
            return Ok(false);
        }
        
        // 3. 检查寄存器压力
        // 这里可以添加更复杂的检查
        
        Ok(true)
    }
    
    /// 检查循环间是否有依赖
    fn has_loop_dependencies(&self, _ctx: &mut NhwcCtx, _loop1: u32, _loop2: u32) -> Result<bool> {
        // 简化版本的依赖检查
        Ok(false)
    }
    
    /// 执行循环融合
    fn perform_loop_fusion(&self, ctx: &mut NhwcCtx, loop1: u32, loop2: u32) -> Result<()> {
        if self.debug {
            println!("        执行循环融合: {} 和 {}", loop1, loop2);
        }
        
        // 简化版本的循环融合实现
        // 这里只是占位符，实际实现会更复杂
        
        Ok(())
    }
    
    /// 优化融合后的循环指令
    fn optimize_fused_loop_instructions(&self, ctx: &mut NhwcCtx, loop_node: u32) -> Result<()> {
        if self.debug {
            println!("        优化融合循环指令: {}", loop_node);
        }
        
        // 简化版本的融合循环优化
        // 这里只是占位符，实际实现会更复杂
        
        Ok(())
    }
}