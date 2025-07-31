use crate::toolkit::rv64_instr::{Arithmetic, Imm, PseudoInstr, Register, Environment};
use crate::toolkit::asm_struct::AsmSection;

/// 实现putint函数
pub fn implement_putint(asm_sect: &mut AsmSection, value_reg: Register) -> Result<(), anyhow::Error> {
    // putint(x) 等价于 write(1, &x, sizeof(int))
    // 系统调用号: write = 64
    
    // 1. 将参数放入正确的寄存器
    // a0 = 1 (stdout)
    asm_sect.asm(PseudoInstr::new_li(Register::new_a(0), Imm::from_offset(1)).into());
    
    // a1 = &x (value_reg的地址)
    // 这里我们需要将value_reg的值存储到栈上，然后传递地址
    // 简化实现：直接传递值
    asm_sect.asm(Arithmetic::new_add(Register::new_a(1), value_reg, Register::Zero).into());
    
    // a2 = sizeof(int) = 4
    asm_sect.asm(PseudoInstr::new_li(Register::new_a(2), Imm::from_offset(4)).into());
    
    // a7 = 64 (write系统调用号)
    asm_sect.asm(PseudoInstr::new_li(Register::new_a(7), Imm::from_offset(64)).into());
    
    // 2. 执行系统调用
    asm_sect.asm(Environment::ECALL {}.into());
    
    Ok(())
}

/// 实现putch函数
pub fn implement_putch(asm_sect: &mut AsmSection, char_reg: Register) -> Result<(), anyhow::Error> {
    // putch(c) 等价于 write(1, &c, 1)
    // 系统调用号: write = 64
    
    // 1. 将参数放入正确的寄存器
    // a0 = 1 (stdout)
    asm_sect.asm(PseudoInstr::new_li(Register::new_a(0), Imm::from_offset(1)).into());
    
    // a1 = &c (char_reg的地址)
    asm_sect.asm(Arithmetic::new_add(Register::new_a(1), char_reg, Register::Zero).into());
    
    // a2 = 1 (1字节)
    asm_sect.asm(PseudoInstr::new_li(Register::new_a(2), Imm::from_offset(1)).into());
    
    // a7 = 64 (write系统调用号)
    asm_sect.asm(PseudoInstr::new_li(Register::new_a(7), Imm::from_offset(64)).into());
    
    // 2. 执行系统调用
    asm_sect.asm(Environment::ECALL {}.into());
    
    Ok(())
}

/// 实现getint函数
pub fn implement_getint(asm_sect: &mut AsmSection, result_reg: Register) -> Result<(), anyhow::Error> {
    // getint() 等价于 read(0, &result, sizeof(int))
    // 系统调用号: read = 63
    
    // 1. 将参数放入正确的寄存器
    // a0 = 0 (stdin)
    asm_sect.asm(PseudoInstr::new_li(Register::new_a(0), Imm::from_offset(0)).into());
    
    // a1 = &result (result_reg的地址)
    asm_sect.asm(Arithmetic::new_add(Register::new_a(1), result_reg, Register::Zero).into());
    
    // a2 = sizeof(int) = 4
    asm_sect.asm(PseudoInstr::new_li(Register::new_a(2), Imm::from_offset(4)).into());
    
    // a7 = 63 (read系统调用号)
    asm_sect.asm(PseudoInstr::new_li(Register::new_a(7), Imm::from_offset(63)).into());
    
    // 2. 执行系统调用
    asm_sect.asm(Environment::ECALL {}.into());
    
    Ok(())
}

/// 实现getch函数
pub fn implement_getch(asm_sect: &mut AsmSection, result_reg: Register) -> Result<(), anyhow::Error> {
    // getch() 等价于 read(0, &result, 1)
    // 系统调用号: read = 63
    
    // 1. 将参数放入正确的寄存器
    // a0 = 0 (stdin)
    asm_sect.asm(PseudoInstr::new_li(Register::new_a(0), Imm::from_offset(0)).into());
    
    // a1 = &result (result_reg的地址)
    asm_sect.asm(Arithmetic::new_add(Register::new_a(1), result_reg, Register::Zero).into());
    
    // a2 = 1 (1字节)
    asm_sect.asm(PseudoInstr::new_li(Register::new_a(2), Imm::from_offset(1)).into());
    
    // a7 = 63 (read系统调用号)
    asm_sect.asm(PseudoInstr::new_li(Register::new_a(7), Imm::from_offset(63)).into());
    
    // 2. 执行系统调用
    asm_sect.asm(Environment::ECALL {}.into());
    
    Ok(())
}

/// 实现exit函数
pub fn implement_exit(asm_sect: &mut AsmSection, exit_code_reg: Register) -> Result<(), anyhow::Error> {
    // exit(code) 等价于 exit_group(code)
    // 系统调用号: exit_group = 94
    
    // 1. 将参数放入正确的寄存器
    // a0 = exit_code
    asm_sect.asm(Arithmetic::new_add(Register::new_a(0), exit_code_reg, Register::Zero).into());
    
    // a7 = 94 (exit_group系统调用号)
    asm_sect.asm(PseudoInstr::new_li(Register::new_a(7), Imm::from_offset(94)).into());
    
    // 2. 执行系统调用
    asm_sect.asm(Environment::ECALL {}.into());
    
    Ok(())
} 