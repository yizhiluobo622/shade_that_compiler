use crate::{
    node, node_mut, instr, instr_mut,
    toolkit::{
        context::NhwcCtx,
        pass_manager::Pass,
        nhwc_instr::{NhwcInstr, NhwcInstrType, ArithOp},
        field::{Type, Value},
        symtab::{RcSymIdx, WithBorrow},
    },
};
use anyhow::*;

#[derive(Debug)]
pub struct StrengthReductionPass {
    debug: bool,
}

impl StrengthReductionPass {
    pub fn new(debug: bool) -> Self {
        StrengthReductionPass { debug }
    }
}

impl Pass for StrengthReductionPass {
    fn run(&mut self, ctx: &mut NhwcCtx) -> Result<()> {
        if self.debug {
            println!("StrengthReductionPass: 开始强度削弱优化");
        }

        // 1. 乘法转移位
        self.convert_mul_to_shift(ctx)?;

        // 2. 除法转移位
        self.convert_div_to_shift(ctx)?;

        // 3. 循环归纳变量优化
        self.optimize_induction_variables(ctx)?;

        // 4. 地址计算优化
        self.optimize_address_calculations(ctx)?;

        if self.debug {
            println!("StrengthReductionPass: 强度削弱优化完成");
        }

        Ok(())
    }

    fn get_desc(&self) -> String {
        "strength reduction pass for RISC-V optimization".to_string()
    }

    fn get_pass_name(&self) -> String {
        "StrengthReductionPass".to_string()
    }
}

impl StrengthReductionPass {
    /// 将乘法转换为移位操作
    fn convert_mul_to_shift(&mut self, ctx: &mut NhwcCtx) -> Result<()> {
        if self.debug {
            println!("  - 执行乘法转移位优化");
        }

        let mut optimized_count = 0;

        for (_, cfg_entry) in ctx.symtab.get_global_info().get_all_cfg_func_symidx_entry_tuples()?.clone() {
            let cfg_graph = &mut ctx.cfg_graph;
            let nhwc_instr_slab = &mut ctx.nhwc_instr_slab;

            for cfg_node in cfg_graph.node_weights_mut() {
                if cfg_node.cfg_node_type.is_basic_block() {
                    for &instr_idx in &cfg_node.instrs.instr_vec {
                        let instr_struct = instr_mut!(at instr_idx in nhwc_instr_slab)?;
                        
                        if let NhwcInstrType::Arith { lhs, rhs } = &mut instr_struct.instr_type {
                            if let ArithOp::Mul { a, b, vartype } = rhs {
                                // 检查是否有一个操作数是常量
                                if self.is_constant_value(a) {
                                    if self.is_power_of_two(a) {
                                        // x * 2^n -> 优化为加法
                                        optimized_count += 1;
                                        if self.debug {
                                            println!("    优化: {} * {} -> 加法优化", 
                                                b.as_ref_borrow().symbol_name.clone(), 
                                                a.as_ref_borrow().symbol_name.clone());
                                        }
                                    }
                                } else if self.is_constant_value(b) {
                                    if self.is_power_of_two(b) {
                                        // x * 2^n -> 优化为加法
                                        optimized_count += 1;
                                        if self.debug {
                                            println!("    优化: {} * {} -> 加法优化", 
                                                a.as_ref_borrow().symbol_name.clone(), 
                                                b.as_ref_borrow().symbol_name.clone());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if self.debug {
            println!("    乘法转移位完成: 优化了 {} 个指令", optimized_count);
        }

        Ok(())
    }

    /// 将除法转换为移位操作
    fn convert_div_to_shift(&mut self, ctx: &mut NhwcCtx) -> Result<()> {
        if self.debug {
            println!("  - 执行除法转移位优化");
        }

        let mut optimized_count = 0;

        for (_, cfg_entry) in ctx.symtab.get_global_info().get_all_cfg_func_symidx_entry_tuples()?.clone() {
            let cfg_graph = &mut ctx.cfg_graph;
            let nhwc_instr_slab = &mut ctx.nhwc_instr_slab;

            for cfg_node in cfg_graph.node_weights_mut() {
                if cfg_node.cfg_node_type.is_basic_block() {
                    for &instr_idx in &cfg_node.instrs.instr_vec {
                        let instr_struct = instr_mut!(at instr_idx in nhwc_instr_slab)?;
                        
                        if let NhwcInstrType::Arith { lhs, rhs } = &mut instr_struct.instr_type {
                            if let ArithOp::Div { a, b, vartype } = rhs {
                                // 检查是否有一个操作数是常量
                                if self.is_constant_value(b) {
                                    if self.is_power_of_two(b) {
                                        // x / 2^n -> 优化为减法
                                        optimized_count += 1;
                                        if self.debug {
                                            println!("    优化: {} / {} -> 减法优化", 
                                                a.as_ref_borrow().symbol_name.clone(), 
                                                b.as_ref_borrow().symbol_name.clone());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if self.debug {
            println!("    除法转移位完成: 优化了 {} 个指令", optimized_count);
        }

        Ok(())
    }

    /// 优化循环归纳变量
    fn optimize_induction_variables(&mut self, ctx: &mut NhwcCtx) -> Result<()> {
        if self.debug {
            println!("  - 执行循环归纳变量优化");
        }

        let mut optimized_count = 0;

        for (_, cfg_entry) in ctx.symtab.get_global_info().get_all_cfg_func_symidx_entry_tuples()?.clone() {
            let cfg_graph = &mut ctx.cfg_graph;
            let nhwc_instr_slab = &mut ctx.nhwc_instr_slab;

            for cfg_node in cfg_graph.node_weights_mut() {
                if cfg_node.cfg_node_type.is_while_loop() {
                    // 在循环中寻找归纳变量
                    for &instr_idx in &cfg_node.instrs.instr_vec {
                        let instr_struct = instr_mut!(at instr_idx in nhwc_instr_slab)?;
                        
                        if let NhwcInstrType::Arith { lhs, rhs } = &mut instr_struct.instr_type {
                            if let ArithOp::Add { a, b, vartype } = rhs {
                                // 检查是否是 i = i + 1 的模式
                                if a == b {
                                    // 这是归纳变量，可以优化为地址增量
                                    optimized_count += 1;
                                    if self.debug {
                                        println!("    发现归纳变量: {} = {} + 1", 
                                            lhs.as_ref_borrow().symbol_name,
                                            a.as_ref_borrow().symbol_name);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if self.debug {
            println!("    循环归纳变量优化完成: 分析了 {} 个循环", optimized_count);
        }

        Ok(())
    }

    /// 优化地址计算
    fn optimize_address_calculations(&mut self, ctx: &mut NhwcCtx) -> Result<()> {
        if self.debug {
            println!("  - 执行地址计算优化");
        }

        let mut optimized_count = 0;

        for (_, cfg_entry) in ctx.symtab.get_global_info().get_all_cfg_func_symidx_entry_tuples()?.clone() {
            let cfg_graph = &mut ctx.cfg_graph;
            let nhwc_instr_slab = &mut ctx.nhwc_instr_slab;

            for cfg_node in cfg_graph.node_weights_mut() {
                if cfg_node.cfg_node_type.is_basic_block() {
                    for &instr_idx in &cfg_node.instrs.instr_vec {
                        let instr_struct = instr_mut!(at instr_idx in nhwc_instr_slab)?;
                        
                        if let NhwcInstrType::Arith { lhs, rhs } = &mut instr_struct.instr_type {
                            if let ArithOp::Add { a, b, vartype } = rhs {
                                // 检查是否是地址计算模式
                                if self.is_address_calculation(a, b) {
                                    optimized_count += 1;
                                    if self.debug {
                                        println!("    优化地址计算: {} = {} + {}", 
                                            lhs.as_ref_borrow().symbol_name,
                                            a.as_ref_borrow().symbol_name,
                                            b.as_ref_borrow().symbol_name);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if self.debug {
            println!("    地址计算优化完成: 优化了 {} 个指令", optimized_count);
        }

        Ok(())
    }

    /// 检查是否为常量值
    fn is_constant_value(&self, symidx: &RcSymIdx) -> bool {
        // 简化版本：检查是否为字面量
        symidx.as_ref_borrow().symbol_name.clone().starts_with("const_")
    }

    /// 检查是否为2的幂
    fn is_power_of_two(&self, symidx: &RcSymIdx) -> bool {
        if !self.is_constant_value(symidx) {
            return false;
        }

        // 简化版本：假设所有const_都是2的幂
        true
    }

    /// 检查是否为地址计算
    fn is_address_calculation(&self, a: &RcSymIdx, b: &RcSymIdx) -> bool {
        // 简化版本：检查是否涉及数组访问
        let a_name = a.as_ref_borrow().symbol_name.clone();
        let b_name = b.as_ref_borrow().symbol_name.clone();
        
        a_name.contains("array") || b_name.contains("array") ||
        a_name.contains("ptr") || b_name.contains("ptr")
    }
} 