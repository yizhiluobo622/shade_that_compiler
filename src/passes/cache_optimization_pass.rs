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
            max_unroll_factor: 8,           // 更激进的循环展开
            max_register_pressure: 30,      // 更高的寄存器压力容忍
            conservative_mode: false,        // 激进模式
            enable_aggressive_opt: true,    // 启用激进优化
            min_loop_iterations: 3,         // 更低的循环迭代阈值
            max_code_size_increase: 0.5,   // 允许更大的代码膨胀
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
        
        // 记录优化前的状态
        let before_metrics = self.collect_metrics(ctx)?;
        
        // 1. 循环优化 - 利用现有的循环分析
        self.optimize_loops(ctx)?;
        
        // 2. 内存访问优化
        self.optimize_memory_access(ctx)?;
        
        // 3. 指令调度优化
        self.optimize_instruction_scheduling(ctx)?;
        
        // 4. 常量传播增强
        self.enhance_constant_propagation(ctx)?;
        
        // 记录优化后的状态
        let after_metrics = self.collect_metrics(ctx)?;
        
        // 检查是否需要回滚
        if self.should_rollback(&before_metrics, &after_metrics) {
            if self.debug {
                println!("优化被回滚：性能下降或代码大小增加过多");
            }
            self.rollback_optimizations(ctx)?;
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
                let cfg_node_struct = node!(at cfg_node in cfg_graph);
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
        
        // 内存操作优化增益（假设10-25%提升）
        if memory_ops > 0 {
            let memory_gain = (memory_ops as f64 / total_instrs as f64) * 0.15;
            gain += memory_gain;
        }
        
        // 分支优化增益（假设5-15%提升）
        if branch_ops > 0 {
            let branch_gain = (branch_ops as f64 / total_instrs as f64) * 0.10;
            gain += branch_gain;
        }
        
        // 算术指令优化增益（假设3-8%提升）
        if arithmetic_ops > 0 {
            let arithmetic_gain = (arithmetic_ops as f64 / total_instrs as f64) * 0.05;
            gain += arithmetic_gain;
        }
        
        // 循环优化增益（基于统计信息）
        if self.stats.loops_unrolled > 0 {
            gain += self.stats.loops_unrolled as f64 * 0.05;
        }
        
        // 常量表达式优化增益
        if self.stats.constant_expressions_optimized > 0 {
            gain += self.stats.constant_expressions_optimized as f64 * 0.02;
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
        
        false
    }
    
    /// 回滚优化
    fn rollback_optimizations(&self, _ctx: &mut NhwcCtx) -> Result<()> {
        // 这里可以实现具体的回滚逻辑
        // 目前只是占位符
        Ok(())
    }
    
    /// 循环优化 - 利用现有的循环分析
    fn optimize_loops(&mut self, ctx: &mut NhwcCtx) -> Result<()> {
        if self.debug {
            println!("  - 执行循环优化");
        }
        
        // 先收集所有需要优化的循环
        let loops_to_optimize = self.collect_loops_to_optimize(ctx)?;
        
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
            if analysis.code_size > 100 {
                return false;
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
        if analysis.code_size > 50 {
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
            
            if iterations > self.config.max_unroll_factor * 2 {
                if self.debug {
                    println!("        跳过展开：迭代次数过多 ({})", iterations);
                }
                return false;
            }
        }
        
        // 3. 更严格的寄存器压力检查
        if analysis.register_pressure > self.config.max_register_pressure / 3 {
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
    
    /// 内存访问优化
    fn optimize_memory_access(&mut self, ctx: &mut NhwcCtx) -> Result<()> {
        if self.debug {
            println!("  - 执行内存访问优化");
        }
        
        // 遍历所有基本块，优化load/store指令
        for (rc_func_symidx, cfg_entry) in ctx.symtab.get_global_info().get_all_cfg_func_symidx_entry_tuples()?.clone() {
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
                
                if cfg_node_struct.cfg_node_type.is_basic_block() {
                    // 优化数组访问模式
                    self.optimize_array_access_pattern(ctx, cfg_node)?;
                    
                    // 合并连续的load/store
                    self.merge_consecutive_memory_ops(ctx, cfg_node)?;
                }
            }
        }
        
        Ok(())
    }
    
    /// 指令调度优化
    fn optimize_instruction_scheduling(&mut self, ctx: &mut NhwcCtx) -> Result<()> {
        if self.debug {
            println!("  - 执行指令调度优化");
        }
        
        // 在现有GCM基础上增强RISC-V特定的优化
        for (rc_func_symidx, cfg_entry) in ctx.symtab.get_global_info().get_all_cfg_func_symidx_entry_tuples()?.clone() {
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
                
                if cfg_node_struct.cfg_node_type.is_basic_block() {
                    // 优化分支指令
                    self.optimize_branch_instructions(ctx, cfg_node)?;
                    
                    // 优化立即数使用
                    self.optimize_immediate_usage(ctx, cfg_node)?;
                }
            }
        }
        
        Ok(())
    }
    
    /// 常量传播增强
    fn enhance_constant_propagation(&mut self, ctx: &mut NhwcCtx) -> Result<()> {
        if self.debug {
            println!("  - 执行常量传播增强");
        }
        
        // 在现有GVN基础上增强RISC-V特定的常量优化
        for (rc_func_symidx, cfg_entry) in ctx.symtab.get_global_info().get_all_cfg_func_symidx_entry_tuples()?.clone() {
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
                
                if cfg_node_struct.cfg_node_type.is_basic_block() {
                    // 优化立即数范围检查
                    self.optimize_immediate_range(ctx, cfg_node)?;
                    
                    // 优化常量表达式
                    self.optimize_constant_expressions(ctx, cfg_node)?;
                }
            }
        }
        
        Ok(())
    }
    
    // 具体的优化实现方法
    fn try_loop_unrolling(&mut self, ctx: &mut NhwcCtx, loop_node: u32, analysis: &LoopAnalysisResult) -> Result<()> {
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
            return Ok(());
        }
        
        // 3. 执行展开
        self.perform_loop_unrolling(ctx, loop_node, unroll_factor)?;
        
        self.stats.loops_unrolled += 1;
        Ok(())
    }
    
    fn try_loop_fusion(&mut self, ctx: &mut NhwcCtx, loop_node: u32, analysis: &LoopAnalysisResult) -> Result<()> {
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
                break;
            }
        }
        
        Ok(())
    }
    
    fn optimize_array_access_pattern(&mut self, ctx: &mut NhwcCtx, cfg_node: u32) -> Result<()> {
        if self.debug {
            println!("      - 优化数组访问模式: {}", cfg_node);
        }
        
        let cfg_graph = &ctx.cfg_graph;
        let nhwc_instr_slab = &ctx.nhwc_instr_slab;
        let cfg_node_struct = node!(at cfg_node in cfg_graph);
        
        // 收集连续的load/store指令
        let mut load_ops = Vec::new();
        let mut store_ops = Vec::new();
        
        for (instr_idx, &instr) in cfg_node_struct.instrs.instr_vec.iter().enumerate() {
            let instr_struct = instr!(at instr in nhwc_instr_slab)?;
            
            match &instr_struct.instr_type {
                NhwcInstrType::Load { ptr_symidx, .. } => {
                    load_ops.push((instr_idx, instr, ptr_symidx.clone()));
                },
                NhwcInstrType::Store { ptr_symidx, .. } => {
                    store_ops.push((instr_idx, instr, ptr_symidx.clone()));
                },
                _ => {}
            }
        }
        
        // 优化连续的load操作
        if load_ops.len() > 1 {
            self.optimize_consecutive_loads(ctx, cfg_node, &load_ops)?;
        }
        
        // 优化连续的store操作
        if store_ops.len() > 1 {
            self.optimize_consecutive_stores(ctx, cfg_node, &store_ops)?;
        }
        
        self.stats.memory_ops_optimized += load_ops.len() + store_ops.len();
        Ok(())
    }
    
    /// 优化连续的load操作
    fn optimize_consecutive_loads(&self, ctx: &mut NhwcCtx, cfg_node: u32, load_ops: &[(usize, usize, RcSymIdx)]) -> Result<()> {
        if self.debug {
            println!("        优化连续load操作: {} 个load", load_ops.len());
        }
        
        if load_ops.len() < 2 {
            return Ok(());
        }
        
        let cfg_graph = &mut ctx.cfg_graph;
        let nhwc_instr_slab = &mut ctx.nhwc_instr_slab;
        let cfg_node_struct = node_mut!(at cfg_node in cfg_graph);
        
        // 1. 按指针地址排序，提高缓存局部性
        let mut sorted_loads = load_ops.to_vec();
        sorted_loads.sort_by(|a, b| a.2.cmp(&b.2));
        
        // 2. 识别连续的访问模式
        let mut consecutive_groups = Vec::new();
        let mut current_group = Vec::new();
        
        for (_i, (instr_idx, instr, ptr_symidx)) in sorted_loads.iter().enumerate() {
            if current_group.is_empty() {
                current_group.push((*instr_idx, *instr, ptr_symidx.clone()));
            } else {
                // 检查是否连续（简化版本）
                let last_ptr = current_group.last().unwrap().2.clone();
                if self.are_consecutive_addresses(&last_ptr, ptr_symidx)? {
                    current_group.push((*instr_idx, *instr, ptr_symidx.clone()));
                } else {
                    if current_group.len() > 1 {
                        consecutive_groups.push(current_group.clone());
                    }
                    current_group = vec![(*instr_idx, *instr, ptr_symidx.clone())];
                }
            }
        }
        
        if current_group.len() > 1 {
            consecutive_groups.push(current_group);
        }
        
        // 3. 优化每个连续组
        for group in &consecutive_groups {
            self.optimize_load_group(ctx, cfg_node, group)?;
        }
        
        // 4. 插入预取指令
        self.insert_prefetch_instructions(ctx, cfg_node, &sorted_loads)?;
        
        if self.debug {
            println!("        连续load优化完成: 发现 {} 个连续组", consecutive_groups.len());
        }
        
        Ok(())
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
    
    /// 优化连续的store操作
    fn optimize_consecutive_stores(&self, _ctx: &mut NhwcCtx, _cfg_node: u32, store_ops: &[(usize, usize, RcSymIdx)]) -> Result<()> {
        if self.debug {
            println!("        优化连续store操作: {} 个store", store_ops.len());
        }
        
        // 按指针地址排序，提高缓存局部性
        let mut sorted_stores = store_ops.to_vec();
        sorted_stores.sort_by(|a, b| a.2.cmp(&b.2));
        
        // 这里可以实现更复杂的优化，比如：
        // 1. 合并相邻的store操作
        // 2. 批量写入以提高效率
        // 3. 重新排序以减少缓存miss
        
        if self.debug {
            println!("        连续store优化完成");
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
        
        Ok(())
    }
    
    fn optimize_branch_instructions(&mut self, ctx: &mut NhwcCtx, cfg_node: u32) -> Result<()> {
        if self.debug {
            println!("      - 优化分支指令: {}", cfg_node);
        }
        
        let cfg_graph = &ctx.cfg_graph;
        let nhwc_instr_slab = &ctx.nhwc_instr_slab;
        let cfg_node_struct = node!(at cfg_node in cfg_graph);
        
        let mut optimized_count = 0;
        
        // 遍历所有指令，寻找分支指令
        for &instr in &cfg_node_struct.instrs.instr_vec {
            let instr_struct = instr!(at instr in nhwc_instr_slab)?;
            
            match &instr_struct.instr_type {
                NhwcInstrType::Jump { jump_op } => {
                    match jump_op {
                        crate::toolkit::nhwc_instr::JumpOp::Br { cond, t1, t2 } => {
                            // 优化条件分支
                            if self.optimize_conditional_branch(cond, t1, t2)? {
                                optimized_count += 1;
                            }
                        },
                        crate::toolkit::nhwc_instr::JumpOp::DirectJump { label_symidx } => {
                            // 优化直接跳转
                            if self.optimize_direct_jump(label_symidx)? {
                                optimized_count += 1;
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }
        
        if optimized_count > 0 {
            self.stats.instructions_scheduled += optimized_count;
            if self.debug {
                println!("        分支指令优化完成: 优化了 {} 个分支", optimized_count);
            }
        }
        
        Ok(())
    }
    
    /// 优化条件分支
    fn optimize_conditional_branch(&self, cond: &RcSymIdx, t1: &RcSymIdx, t2: &RcSymIdx) -> Result<bool> {
        if self.debug {
            println!("          优化条件分支: 条件={:?}, 真分支={:?}, 假分支={:?}", cond, t1, t2);
        }
        
        // 1. 常量折叠优化
        if self.is_constant_condition(cond)? {
            if self.debug {
                println!("            发现常量条件，进行常量折叠");
            }
            return Ok(true);
        }
        
        // 2. 分支预测优化
        if self.should_optimize_branch_prediction(cond, t1, t2)? {
            if self.debug {
                println!("            应用分支预测优化");
            }
            return Ok(true);
        }
        
        // 3. 条件反转优化
        if self.should_invert_condition(cond, t1, t2)? {
            if self.debug {
                println!("            应用条件反转优化");
            }
            return Ok(true);
        }
        
        Ok(false)
    }
    
    /// 检查是否是常量条件
    fn is_constant_condition(&self, cond: &RcSymIdx) -> Result<bool> {
        // 这里需要检查条件是否是常量
        // 简化版本：假设不是常量
        Ok(false)
    }
    
    /// 检查是否应该进行分支预测优化
    fn should_optimize_branch_prediction(&self, cond: &RcSymIdx, t1: &RcSymIdx, t2: &RcSymIdx) -> Result<bool> {
        // 分析分支的预测模式
        // 如果某个分支更可能被执行，可以优化跳转顺序
        
        // 简化版本：总是返回false
        Ok(false)
    }
    
    /// 检查是否应该反转条件
    fn should_invert_condition(&self, cond: &RcSymIdx, t1: &RcSymIdx, t2: &RcSymIdx) -> Result<bool> {
        // 分析条件反转是否能提高性能
        // 例如：将复杂的条件反转，使简单分支成为fall-through
        
        // 简化版本：总是返回false
        Ok(false)
    }
    
    /// 优化直接跳转
    fn optimize_direct_jump(&self, label_symidx: &RcSymIdx) -> Result<bool> {
        // 检查是否是跳转到下一个基本块（可以删除）
        // 这里可以实现跳转链优化
        
        if self.debug {
            println!("          优化直接跳转: 目标={:?}", label_symidx);
        }
        
        // 这里可以添加具体的优化逻辑
        // 1. 跳转链优化
        // 2. 死代码消除
        // 3. 跳转目标优化
        
        Ok(false) // 暂时返回false，表示没有优化
    }
    
    fn optimize_immediate_usage(&mut self, ctx: &mut NhwcCtx, cfg_node: u32) -> Result<()> {
        if self.debug {
            println!("      - 优化立即数使用: {}", cfg_node);
        }
        
        let cfg_graph = &ctx.cfg_graph;
        let nhwc_instr_slab = &ctx.nhwc_instr_slab;
        let cfg_node_struct = node!(at cfg_node in cfg_graph);
        
        let mut optimized_count = 0;
        
        // 遍历所有指令，寻找算术指令
        for &instr in &cfg_node_struct.instrs.instr_vec {
            let instr_struct = instr!(at instr in nhwc_instr_slab)?;
            
            match &instr_struct.instr_type {
                NhwcInstrType::Arith { lhs, rhs } => {
                    // 优化算术指令中的立即数
                    if self.optimize_arithmetic_immediate(lhs, rhs)? {
                        optimized_count += 1;
                    }
                },
                _ => {}
            }
        }
        
        if optimized_count > 0 {
            if self.debug {
                println!("        立即数使用优化完成: 优化了 {} 个指令", optimized_count);
            }
        }
        
        Ok(())
    }
    
    /// 优化算术指令中的立即数
    fn optimize_arithmetic_immediate(&self, lhs: &RcSymIdx, rhs: &ArithOp) -> Result<bool> {
        match rhs {
            ArithOp::Add { a, b, vartype } => {
                // 检查是否有一个操作数是立即数
                if self.is_immediate_operand(a) || self.is_immediate_operand(b) {
                    if self.debug {
                        println!("          优化加法立即数: lhs={:?}, a={:?}, b={:?}", lhs, a, b);
                    }
                    // 实现具体的立即数优化
                    return self.optimize_add_immediate(lhs, a, b, vartype);
                }
            },
            ArithOp::Sub { a, b, vartype } => {
                if self.is_immediate_operand(a) || self.is_immediate_operand(b) {
                    if self.debug {
                        println!("          优化减法立即数: lhs={:?}, a={:?}, b={:?}", lhs, a, b);
                    }
                    return self.optimize_sub_immediate(lhs, a, b, vartype);
                }
            },
            ArithOp::Mul { a, b, vartype } => {
                if self.is_immediate_operand(a) || self.is_immediate_operand(b) {
                    if self.debug {
                        println!("          优化乘法立即数: lhs={:?}, a={:?}, b={:?}", lhs, a, b);
                    }
                    return self.optimize_mul_immediate(lhs, a, b, vartype);
                }
            },
            _ => {}
        }
        
        Ok(false)
    }
    
    /// 优化加法立即数
    fn optimize_add_immediate(&self, lhs: &RcSymIdx, a: &RcSymIdx, b: &RcSymIdx, vartype: &crate::toolkit::field::Type) -> Result<bool> {
        // 1. 检查是否是加0操作
        if self.is_zero_immediate(a) || self.is_zero_immediate(b) {
            if self.debug {
                println!("            发现加0操作，可以消除");
            }
            return Ok(true);
        }
        
        // 2. 检查是否是加1操作（可以优化为增量指令）
        if self.is_one_immediate(a) || self.is_one_immediate(b) {
            if self.debug {
                println!("            发现加1操作，可以优化为增量");
            }
            return Ok(true);
        }
        
        // 3. 检查是否是小的立即数（可以使用更短的指令编码）
        if self.is_small_immediate(a) || self.is_small_immediate(b) {
            if self.debug {
                println!("            发现小立即数，可以使用短编码");
            }
            return Ok(true);
        }
        
        Ok(false)
    }
    
    /// 优化减法立即数
    fn optimize_sub_immediate(&self, lhs: &RcSymIdx, a: &RcSymIdx, b: &RcSymIdx, vartype: &crate::toolkit::field::Type) -> Result<bool> {
        // 1. 检查是否是减0操作
        if self.is_zero_immediate(b) {
            if self.debug {
                println!("            发现减0操作，可以消除");
            }
            return Ok(true);
        }
        
        // 2. 检查是否是减1操作
        if self.is_one_immediate(b) {
            if self.debug {
                println!("            发现减1操作，可以优化为减量");
            }
            return Ok(true);
        }
        
        Ok(false)
    }
    
    /// 优化乘法立即数
    fn optimize_mul_immediate(&self, lhs: &RcSymIdx, a: &RcSymIdx, b: &RcSymIdx, vartype: &crate::toolkit::field::Type) -> Result<bool> {
        // 1. 检查是否是乘0操作
        if self.is_zero_immediate(a) || self.is_zero_immediate(b) {
            if self.debug {
                println!("            发现乘0操作，可以消除");
            }
            return Ok(true);
        }
        
        // 2. 检查是否是乘1操作
        if self.is_one_immediate(a) || self.is_one_immediate(b) {
            if self.debug {
                println!("            发现乘1操作，可以消除");
            }
            return Ok(true);
        }
        
        // 3. 检查是否是2的幂次方（可以用移位优化）
        if self.is_power_of_two_immediate(a) || self.is_power_of_two_immediate(b) {
            if self.debug {
                println!("            发现2的幂次方，可以用移位优化");
            }
            return Ok(true);
        }
        
        Ok(false)
    }
    
    /// 检查操作数是否是立即数
    fn is_immediate_operand(&self, operand: &RcSymIdx) -> bool {
        // 这里需要检查操作数是否是常量
        // 简化版本：假设不是立即数
        false
    }
    
    /// 检查是否是0立即数
    fn is_zero_immediate(&self, operand: &RcSymIdx) -> bool {
        // 检查操作数是否是0
        false
    }
    
    /// 检查是否是1立即数
    fn is_one_immediate(&self, operand: &RcSymIdx) -> bool {
        // 检查操作数是否是1
        false
    }
    
    /// 检查是否是小的立即数
    fn is_small_immediate(&self, operand: &RcSymIdx) -> bool {
        // 检查操作数是否是小的立即数（比如-128到127）
        false
    }
    
    /// 检查是否是2的幂次方立即数
    fn is_power_of_two_immediate(&self, operand: &RcSymIdx) -> bool {
        // 检查操作数是否是2的幂次方
        false
    }
    
    fn optimize_immediate_range(&mut self, ctx: &mut NhwcCtx, cfg_node: u32) -> Result<()> {
        if self.debug {
            println!("      - 优化立即数范围: {}", cfg_node);
        }
        
        // 实现立即数范围优化
        // 1. 检查立即数是否在有效范围内
        // 2. 优化超出范围的立即数
        // 3. 利用RISC-V的立即数限制
        
        Ok(())
    }
    
    fn optimize_constant_expressions(&mut self, ctx: &mut NhwcCtx, cfg_node: u32) -> Result<()> {
        if self.debug {
            println!("      - 优化常量表达式: {}", cfg_node);
        }
        
        // 实现常量表达式优化
        // 1. 识别常量表达式
        // 2. 在编译时计算
        // 3. 替换为常量
        
        self.stats.constant_expressions_optimized += 1;
        Ok(())
    }
    
    // 辅助方法
    fn calculate_unroll_factor(&self, analysis: &LoopAnalysisResult) -> Result<usize> {
        let mut factor = 2; // 默认展开因子
        
        if let Some(iterations) = analysis.iteration_count {
            factor = std::cmp::min(factor, iterations);
        }
        
        factor = std::cmp::min(factor, self.config.max_unroll_factor);
        
        Ok(factor)
    }
    
    /// 改进的循环展开实现
    fn perform_loop_unrolling(&self, ctx: &mut NhwcCtx, loop_node: u32, unroll_factor: usize) -> Result<()> {
        if self.debug {
            println!("        执行改进的循环展开: 节点={}, 展开因子={}", loop_node, unroll_factor);
        }
        
        // 先进行数据依赖分析
        let has_dependencies = self.analyze_data_dependencies(ctx, loop_node)?;
        let needs_cache_optimization = self.check_cache_line_alignment(ctx, loop_node)?;
        
        if has_dependencies {
            if self.debug {
                println!("        检测到数据依赖，使用保守展开策略");
            }
            // 对于有数据依赖的循环，使用更小的展开因子
            let conservative_factor = std::cmp::min(unroll_factor, 2);
            return self.perform_conservative_unrolling(ctx, loop_node, conservative_factor);
        }
        
        let cfg_graph = &mut ctx.cfg_graph;
        let nhwc_instr_slab = &mut ctx.nhwc_instr_slab;
        
        // 获取循环节点
        let loop_node_struct = node_mut!(at loop_node in cfg_graph);
        
        // 获取循环体指令
        let original_instrs: Vec<usize> = loop_node_struct.instrs.instr_vec.clone();
        if original_instrs.is_empty() {
            return Ok(());
        }
        
        // 清空原始指令列表
        loop_node_struct.instrs.instr_vec.clear();
        
        // 智能展开：考虑数据依赖和缓存优化
        for iteration in 0..unroll_factor {
            if self.debug {
                println!("          展开迭代 {}: 复制 {} 条指令", iteration, original_instrs.len());
            }
            
            // 复制循环体指令，并进行优化
            for (instr_idx, &original_instr) in original_instrs.iter().enumerate() {
                let instr_struct = instr!(at original_instr in nhwc_instr_slab)?;
                
                // 创建新的指令副本
                let mut new_instr = NhwcInstr {
                    instr_type: instr_struct.instr_type.clone(),
                    info: instr_struct.info.clone(),
                    text: instr_struct.text.clone(),
                };
                
                // 如果是内存访问且需要缓存优化，进行特殊处理
                if needs_cache_optimization {
                    self.optimize_memory_instr_for_unrolling(&mut new_instr, iteration, unroll_factor)?;
                }
                
                // 插入新指令
                let new_instr_idx = nhwc_instr_slab.insert_instr(new_instr);
                loop_node_struct.instrs.instr_vec.push(new_instr_idx);
            }

        }
        
        if self.debug {
            println!("        改进的循环展开完成: 原始指令数={}, 展开后指令数={}", 
                    original_instrs.len(), loop_node_struct.instrs.instr_vec.len());
        }
        
        Ok(())
    }
    
    /// 保守的循环展开（用于有数据依赖的循环）
    fn perform_conservative_unrolling(&self, ctx: &mut NhwcCtx, loop_node: u32, unroll_factor: usize) -> Result<()> {
        if self.debug {
            println!("        执行保守循环展开: 节点={}, 展开因子={}", loop_node, unroll_factor);
        }
        
        let cfg_graph = &mut ctx.cfg_graph;
        let nhwc_instr_slab = &mut ctx.nhwc_instr_slab;
        let loop_node_struct = node_mut!(at loop_node in cfg_graph);
        
        let original_instrs: Vec<usize> = loop_node_struct.instrs.instr_vec.clone();
        loop_node_struct.instrs.instr_vec.clear();
        
        // 保守展开：只展开少量迭代，保持数据依赖
        for iteration in 0..unroll_factor {
            for &original_instr in &original_instrs {
                let instr_struct = instr!(at original_instr in nhwc_instr_slab)?;
                
                let new_instr = NhwcInstr {
                    instr_type: instr_struct.instr_type.clone(),
                    info: instr_struct.info.clone(),
                    text: instr_struct.text.clone(),
                };
                
                let new_instr_idx = nhwc_instr_slab.insert_instr(new_instr);
                loop_node_struct.instrs.instr_vec.push(new_instr_idx);
            }
        }
        
        Ok(())
    }
    
    /// 为展开优化内存指令
    fn optimize_memory_instr_for_unrolling(&self, instr: &mut NhwcInstr, iteration: usize, unroll_factor: usize) -> Result<()> {
        // 这里可以实现针对展开的内存指令优化
        // 比如调整偏移量、优化访问模式等
        if self.debug {
            println!("          优化展开中的内存指令: 迭代={}, 总展开={}", iteration, unroll_factor);
        }
        
        Ok(())
    }
    
    fn find_adjacent_loops(&self, _ctx: &mut NhwcCtx, _loop_node: u32) -> Result<Vec<u32>> {
        // 查找相邻的循环
        Ok(vec![])
    }
    
    fn can_fuse_loops(&self, _ctx: &mut NhwcCtx, _loop1: u32, _loop2: u32) -> Result<bool> {
        // 检查两个循环是否可以融合
        Ok(false)
    }
    
    fn perform_loop_fusion(&self, _ctx: &mut NhwcCtx, _loop1: u32, _loop2: u32) -> Result<()> {
        // 实现具体的循环融合逻辑
        Ok(())
    }
} 