use crate::toolkit::{context::NhwcCtx, pass_manager::Pass, dot::Config, etc::generate_png_by_graph_multi_tasks};
use anyhow::*;
use crate::{node, node_mut, instr, instr_mut};
use crate::toolkit::{cfg_node::CFG_ROOT, etc::dfs_with_priority, cfg_edge::CfgEdgeType, nhwc_instr::{NhwcInstrType, ArithOp}};
use crate::toolkit::symtab::{RcSymIdx, WithBorrow};

#[derive(Debug)]
pub struct CacheOptimizationPass {
    debug: bool,
}

impl CacheOptimizationPass {
    pub fn new(debug: bool) -> Self {
        Self { debug }
    }
}

impl Pass for CacheOptimizationPass {
    fn run(&mut self, ctx: &mut NhwcCtx) -> Result<()> {
        if self.debug {
            println!("CacheOptimizationPass: 开始RISC-V缓存优化");
        }
        
        // 1. 循环优化 - 利用现有的循环分析
        self.optimize_loops(ctx)?;
        
        // 2. 内存访问优化
        self.optimize_memory_access(ctx)?;
        
        // 3. 指令调度优化
        self.optimize_instruction_scheduling(ctx)?;
        
        // 4. 常量传播增强
        self.enhance_constant_propagation(ctx)?;
        
        if self.debug {
            println!("CacheOptimizationPass: RISC-V缓存优化完成");
        }
        
        Ok(())
    }
    
    fn get_desc(&self) -> String { 
        return "cache optimization pass for RISC-V".to_string(); 
    }

    fn get_pass_name(&self) -> String { 
        return "CacheOptimizationPass".to_string(); 
    }
}

impl CacheOptimizationPass {
    /// 循环优化 - 利用现有的循环分析
    fn optimize_loops(&self, ctx: &mut NhwcCtx) -> Result<()> {
        if self.debug {
            println!("  - 执行循环优化");
        }
        
        // 遍历所有函数
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
                
                // 检查是否是循环节点
                if cfg_node_struct.cfg_node_type.is_while_loop() {
                    if self.debug {
                        println!("    - 发现循环节点: {}", cfg_node);
                    }
                    
                    // 尝试循环展开
                    self.try_loop_unrolling(ctx, cfg_node)?;
                    
                    // 尝试循环融合
                    self.try_loop_fusion(ctx, cfg_node)?;
                }
            }
        }
        
        Ok(())
    }
    
    /// 内存访问优化
    fn optimize_memory_access(&self, ctx: &mut NhwcCtx) -> Result<()> {
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
    fn optimize_instruction_scheduling(&self, ctx: &mut NhwcCtx) -> Result<()> {
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
    fn enhance_constant_propagation(&self, ctx: &mut NhwcCtx) -> Result<()> {
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
    fn try_loop_unrolling(&self, ctx: &mut NhwcCtx, loop_node: u32) -> Result<()> {
        // 循环展开实现
        // 这里可以调用你现有的loop_optimize.rs中的功能
        if self.debug {
            println!("      - 尝试循环展开: {}", loop_node);
        }
        Ok(())
    }
    
    fn try_loop_fusion(&self, ctx: &mut NhwcCtx, loop_node: u32) -> Result<()> {
        // 循环融合实现
        if self.debug {
            println!("      - 尝试循环融合: {}", loop_node);
        }
        Ok(())
    }
    
    fn optimize_array_access_pattern(&self, ctx: &mut NhwcCtx, cfg_node: u32) -> Result<()> {
        // 优化数组访问模式
        if self.debug {
            println!("      - 优化数组访问模式: {}", cfg_node);
        }
        Ok(())
    }
    
    fn merge_consecutive_memory_ops(&self, ctx: &mut NhwcCtx, cfg_node: u32) -> Result<()> {
        // 合并连续的load/store指令
        if self.debug {
            println!("      - 合并连续内存操作: {}", cfg_node);
        }
        Ok(())
    }
    
    fn optimize_branch_instructions(&self, ctx: &mut NhwcCtx, cfg_node: u32) -> Result<()> {
        // 优化分支指令
        if self.debug {
            println!("      - 优化分支指令: {}", cfg_node);
        }
        Ok(())
    }
    
    fn optimize_immediate_usage(&self, ctx: &mut NhwcCtx, cfg_node: u32) -> Result<()> {
        // 优化立即数使用
        if self.debug {
            println!("      - 优化立即数使用: {}", cfg_node);
        }
        Ok(())
    }
    
    fn optimize_immediate_range(&self, ctx: &mut NhwcCtx, cfg_node: u32) -> Result<()> {
        // 优化立即数范围检查
        if self.debug {
            println!("      - 优化立即数范围: {}", cfg_node);
        }
        Ok(())
    }
    
    fn optimize_constant_expressions(&self, ctx: &mut NhwcCtx, cfg_node: u32) -> Result<()> {
        // 优化常量表达式
        if self.debug {
            println!("      - 优化常量表达式: {}", cfg_node);
        }
        Ok(())
    }
} 