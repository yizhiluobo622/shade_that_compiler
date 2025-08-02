use std::fmt::Debug;
use anyhow::*;
use itertools::Itertools;
use strum_macros::EnumIs;

use crate::{debug_info_blue, debug_info_red, toolkit::rv64_instr::{Arithmetic, BaseIntInstr}};

use super::{field::Value, rv64_instr::{Imm, RV64Instr, Register}, symtab::{RcSymIdx, SymIdx, WithBorrow}};

/// a asm file contains several sections
pub struct AsmStructure{
    pub sects:Vec<AsmSection>
}
impl AsmStructure{
    pub fn new() -> Self{
        Self { sects: vec![] }
    }
    pub fn dump(&self,enable_annotation:bool) -> String{
        let built_in_part = "# Built-in library\n .text\n .align 4\n .globl starttime\n .type starttime, @function\n starttime:\n mv a0, zero\n tail _sysy_starttime\n \n .text\n .align 4\n .globl stoptime\n .type stoptime, @function\n stoptime:\n mv a0, zero\n tail _sysy_stoptime\n\n\n";
        // let built_in_part = "";
        let mut s = String::new();
        for sect in &self.sects{
            s += format!("{}",sect.dump(enable_annotation)).as_str();
        }
        format!("{}{}",built_in_part,s)
    }
}
impl AsmSection{
    pub fn new(name:String) -> Self{
        Self{
            sect_name: name,
            stmts: Vec::new(),
            symtab: None,
            enable_division_optimization: true  // 重新启用除法优化
        }
    }
    
    pub fn new_with_symtab(name:String, symtab: std::rc::Rc<std::cell::RefCell<super::symtab::SymTab>>) -> Self{
        Self{
            sect_name: name,
            stmts: Vec::new(),
            symtab: Some(symtab),
            enable_division_optimization: true  // 重新启用除法优化
        }
    }
    pub fn annotate(&mut self, annotation:String){
        //self.stmts.push(AsmAttr::Annotation { annotation }.into())
    }
    pub fn global(&mut self, imm:Imm){
        self.stmts.push(AsmAttr::Global { label: imm }.into())
    }
    pub fn obj_type(&mut self, imm:Imm, ){
        self.stmts.push(AsmAttr::DataType { attr_ty:DataType::Object,  imm } .into())
    }
    pub fn func_type(&mut self, imm:Imm){
        self.stmts.push(AsmAttr::DataType { attr_ty:DataType::Function,  imm} .into())
    }
    pub fn label(&mut self, imm:Imm){
        self.stmts.push(AsmAttr::Label  { imm }.into())
    }
    pub fn double(&mut self, imm:Imm){
        self.stmts.push(AsmAttr::Double { imm: imm }.into())
    }
    pub fn word(&mut self, imm:Imm){
        self.stmts.push(AsmAttr::Word { imm: imm }.into())
    }
    pub fn half(&mut self, imm:Imm){
        self.stmts.push(AsmAttr::Half { imm: imm }.into())
    }
    pub fn byte(&mut self, imm:Imm){
        self.stmts.push(AsmAttr::Byte { imm: imm }.into())
    }
    pub fn zero(&mut self, len:usize){
        if len >0 {
            self.stmts.push(AsmAttr::Zero { len }.into())
        }
    }
    pub fn align(&mut self, align:usize){
        self.stmts.push(AsmAttr::Align { align } .into())
    }
    pub fn data(&mut self){
        self.stmts.push(AsmAttr::Data {  } .into())
    }
    pub fn text(&mut self){
        self.stmts.push(Asm::Attr { attr: AsmAttr::Text{} })
    }
    
    /// 启用除法优化
    pub fn enable_division_optimization(&mut self) {
        self.enable_division_optimization = true;
    }
    
    /// 禁用除法优化
    pub fn disable_division_optimization(&mut self) {
        self.enable_division_optimization = false;
    }
    
    /// 设置除法优化状态
    pub fn set_division_optimization(&mut self, enabled: bool) {
        self.enable_division_optimization = enabled;
    }
    pub fn asm(&mut self, riscv_instr:RV64Instr){
        debug_info_blue!("asm:{:?}", riscv_instr);
        //合并乘加
        match self.stmts.last(){
            Some(former_asm) => {
                match former_asm {
                    Asm::Attr { attr } => () ,
                    Asm::Riscv { instr } => {
                        println!("{:?}",instr);
                        match instr{
                            RV64Instr::BaseIntInstr(baseintinstr)=>{
                                //println!("{:?}",baseintinstr);
                                match baseintinstr {           
                                    BaseIntInstr::Arithmetic(arithmetic) => {
                                        match arithmetic {
                                            
                                            Arithmetic::FMULS { rd: mul_rd, rs1: mul_rs1, rs2: mul_rs2 } => {
                                                 println!("匹配到FMULS指令!");
                                                 
                                                 // 检查当前指令是否是浮点加法
                                                 match &riscv_instr {
                                                     RV64Instr::BaseIntInstr(super::rv64_instr::BaseIntInstr::Arithmetic(super::rv64_instr::Arithmetic::FADDS { rd: add_rd, rs1: add_rs1, rs2: add_rs2 })) => {
                                                         // 检查是否可以合并为FMADD: add_rs1 == mul_rd && add_rs2 != mul_rd
                                                         if add_rs1 == mul_rd && add_rs2 != mul_rd {
                                                             println!("优化: FMULS + FADDS -> FMADD");
                                                             
                                                             // 创建FMADD指令
                                                             let fmadd_instr = RV64Instr::BaseIntInstr(super::rv64_instr::BaseIntInstr::MulAdd(super::rv64_instr::MulAdd::Fmadds {
                                                                 rd: add_rd.clone(),
                                                                 rs1: mul_rs1.clone(),
                                                                 rs2: mul_rs2.clone(),
                                                                 rs3: add_rs2.clone(),
                                                             }));
                                                             
                                                             // 替换最后一条指令（FMULS）
                                                             if let Some(last_stmt) = self.stmts.last_mut() {
                                                                 if let Asm::Riscv { instr } = last_stmt {
                                                                     *instr = fmadd_instr;
                                                                 }
                                                             }
                                                             
                                                             return; // 不添加新指令，因为已经合并了
                                                         }
                                                     },
                                                     RV64Instr::BaseIntInstr(super::rv64_instr::BaseIntInstr::Arithmetic(super::rv64_instr::Arithmetic::FSUBS { rd: sub_rd, rs1: sub_rs1, rs2: sub_rs2 })) => {
                                                         // 检查是否可以合并为FMSUB: sub_rs1 == mul_rd && sub_rs2 != mul_rd
                                                         if sub_rs1 == mul_rd && sub_rs2 != mul_rd {
                                                             println!("优化: FMULS + FSUBS -> FMSUB");
                                                             
                                                             // 创建FMSUB指令
                                                             let fmsub_instr = RV64Instr::BaseIntInstr(super::rv64_instr::BaseIntInstr::MulAdd(super::rv64_instr::MulAdd::Fmsubs {
                                                                 rd: sub_rd.clone(),
                                                                 rs1: mul_rs1.clone(),
                                                                 rs2: mul_rs2.clone(),
                                                                 rs3: sub_rs2.clone(),
                                                             }));
                                                             
                                                             // 替换最后一条指令（FMULS）
                                                             if let Some(last_stmt) = self.stmts.last_mut() {
                                                                 if let Asm::Riscv { instr } = last_stmt {
                                                                     *instr = fmsub_instr;
                                                                 }
                                                             }
                                                             
                                                             return; // 不添加新指令，因为已经合并了
                                                         }
                                                     },
                                                     _ => {}
                                                 }
                                             },
                                            _ => (),
                                        }
                                                                         },
                                     super::rv64_instr::BaseIntInstr::MulAdd(muladd) => {
                                         match muladd {
                                             super::rv64_instr::MulAdd::Fnmadds { rd: mul_rd, rs1: mul_rs1, rs2: mul_rs2, rs3: mul_rs3 } => {
                                                 println!("匹配到FNMULS指令!");
                                                 
                                                 // 检查当前指令是否是浮点加法或减法
                                                 match &riscv_instr {
                                                     RV64Instr::BaseIntInstr(super::rv64_instr::BaseIntInstr::Arithmetic(super::rv64_instr::Arithmetic::FADDS { rd: add_rd, rs1: add_rs1, rs2: add_rs2 })) => {
                                                         if add_rs1 == mul_rd && add_rs2 != mul_rd {
                                                             println!("优化: FNMULS + FADDS -> FNMADD");
                                                             
                                                             let fnmadd_instr = RV64Instr::BaseIntInstr(super::rv64_instr::BaseIntInstr::MulAdd(super::rv64_instr::MulAdd::Fnmadds {
                                                                 rd: add_rd.clone(),
                                                                 rs1: mul_rs1.clone(),
                                                                 rs2: mul_rs2.clone(),
                                                                 rs3: add_rs2.clone(),
                                                             }));
                                                             
                                                             if let Some(last_stmt) = self.stmts.last_mut() {
                                                                 if let Asm::Riscv { instr } = last_stmt {
                                                                     *instr = fnmadd_instr;
                                                                 }
                                                             }
                                                             return;
                                                         }
                                                     },
                                                     RV64Instr::BaseIntInstr(super::rv64_instr::BaseIntInstr::Arithmetic(super::rv64_instr::Arithmetic::FSUBS { rd: sub_rd, rs1: sub_rs1, rs2: sub_rs2 })) => {
                                                         if sub_rs1 == mul_rd && sub_rs2 != mul_rd {
                                                             println!("优化: FNMULS + FSUBS -> FNMSUB");
                                                             
                                                             let fnmsub_instr = RV64Instr::BaseIntInstr(super::rv64_instr::BaseIntInstr::MulAdd(super::rv64_instr::MulAdd::Fnmsubs {
                                                                 rd: sub_rd.clone(),
                                                                 rs1: mul_rs1.clone(),
                                                                 rs2: mul_rs2.clone(),
                                                                 rs3: sub_rs2.clone(),
                                                             }));
                                                             
                                                             if let Some(last_stmt) = self.stmts.last_mut() {
                                                                 if let Asm::Riscv { instr } = last_stmt {
                                                                     *instr = fnmsub_instr;
                                                                 }
                                                             }
                                                             return;
                                                         }
                                                     },
                                                     _ => {}
                                                 }
                                             },
                                             _ => {}
                                         }
                                     },
                                     _ => {},
                                 }
                            }
                                                      
                            _ => {},
                        }
                    },
                }
            },
            None => todo!(),
        }
        //合并取反
        if self.stmts.len() >= 2{
        match &self.stmts[self.stmts.len() - 2]{
            Asm::Attr { attr } => (),
            Asm::Riscv { instr } => {
                println!("{:?}",instr);
                match instr{
                    RV64Instr::BaseIntInstr(base_int_instr) => (),
                    RV64Instr::PseudoInstr(pseudo_instr) => {
                        match pseudo_instr{  
                            //
                            //check whether it is Li (len-2)
                            //                       
                            crate::toolkit::rv64_instr::PseudoInstr::Li { rd, imm } => {
                                match  &self.stmts[self.stmts.len() - 1]{
                                    Asm::Attr { attr } => (),
                                    Asm::Riscv { instr } => {
                                        match instr{
                                            RV64Instr::BaseIntInstr(base_int_instr) => (),
                                            RV64Instr::PseudoInstr(pseudo_instr) => {
                                                match pseudo_instr {
                                                    //
                                                    //check whether it is Fmv_w_x (len-1)
                                                    //
                                                    crate::toolkit::rv64_instr::PseudoInstr::Fmv_w_x { rd: fmv_rd, rs: fmv_rs } => {
                                                        // 检查当前指令是否是fsub.s
                                                        match &riscv_instr {
                                                            RV64Instr::BaseIntInstr(super::rv64_instr::BaseIntInstr::Arithmetic(super::rv64_instr::Arithmetic::FSUBS { rd: sub_rd, rs1: sub_rs1, rs2: sub_rs2 })) => {
                                                                // 检查是否可以合并为Fneg_s: li(0) + fmv.w.x(fa0,a0) + fsub.s(fa3,fa0,fa2) -> fneg.s fa3,fa2
                                                                if fmv_rs == rd && sub_rs1 == fmv_rd && sub_rs2 != fmv_rd {
                                                                    println!("优化: LI(0) + FMV_W_X + FSUBS -> FNEG_S");
                                                                    
                                                                    // 创建Fneg_s指令
                                                                    let fneg_instr = RV64Instr::PseudoInstr(super::rv64_instr::PseudoInstr::Fneg_s {
                                                                        rd: sub_rd.clone(),
                                                                        rs: sub_rs2.clone(),
                                                                    });
                                                                    
                                                                    // 替换最后一条指令（FSUBS）
                                                                    if let Some(last_stmt) = self.stmts.last_mut() {
                                                                        if let Asm::Riscv { instr } = last_stmt {
                                                                            *instr = fneg_instr;
                                                                        }
                                                                    }
                                                                    self.stmts.remove(self.stmts.len() - 2);
                                                                    return; // 不添加新指令，因为已经合并了
                                                                }
                                                            },
                                                            _ => {}
                                                        }
                                                    },
                                                    _ => ()
                                                }
                                            }
                                        }
                                    },
                                }
                            },                            
                            _ =>(),
                        }
                    },
                }
                        
            },
            
        }
        }
        //合并乘积取反
        match &self.stmts[self.stmts.len() - 1]{
            Asm::Attr { attr } => (),
            Asm::Riscv { instr } => {
                match instr{
                    RV64Instr::BaseIntInstr(base_int_instr) => (),
                    RV64Instr::PseudoInstr(pseudo_instr) => {
                        match pseudo_instr{ 

                            crate::toolkit::rv64_instr::PseudoInstr::Fneg_s { rd: fneg_rd, rs: fneg_rs } => {
                                // 检查当前指令是否是浮点加法或减法
                                match &riscv_instr {
                                    RV64Instr::BaseIntInstr(super::rv64_instr::BaseIntInstr::Arithmetic(super::rv64_instr::Arithmetic::FADDS { rd: add_rd, rs1: add_rs1, rs2: add_rs2 })) => {
                                        // 检查是否可以合并为Fnmadd: fneg_s(rd,rs) + fadd.s(rd,rd,rs2) -> fnmadd.s rd,rs,rs2,rs2
                                                                                                 if add_rs1 == fneg_rd && add_rs2 != fneg_rd && add_rd != fneg_rd {
                                            println!("优化: FNEG_S + FADDS -> FNMADD");
                                            
                                            // 创建Fnmadds指令
                                            let fnmadd_instr = RV64Instr::BaseIntInstr(super::rv64_instr::BaseIntInstr::MulAdd(super::rv64_instr::MulAdd::Fnmadds {
                                                rd: add_rd.clone(),
                                                rs1: fneg_rs.clone(),
                                                rs2: add_rs2.clone(),
                                                rs3: add_rs2.clone(),
                                            }));
                                            
                                            // 替换最后一条指令（FNEG_S）
                                            if let Some(last_stmt) = self.stmts.last_mut() {
                                                if let Asm::Riscv { instr } = last_stmt {
                                                    *instr = fnmadd_instr;
                                                }
                                            }
                                            
                                            return; // 不添加新指令，因为已经合并了
                                        }
                                    },
                                    RV64Instr::BaseIntInstr(super::rv64_instr::BaseIntInstr::Arithmetic(super::rv64_instr::Arithmetic::FSUBS { rd: sub_rd, rs1: sub_rs1, rs2: sub_rs2 })) => {
                                        // 检查是否可以合并为Fnmsub: fneg_s(rd,rs) + fsub.s(rd,rd,rs2) -> fnmsub.s rd,rs,rs2,rs2
                                                                                                 if sub_rs1 == fneg_rd && sub_rs2 != fneg_rd && sub_rd != fneg_rd {
                                            println!("优化: FNEG_S + FSUBS -> FNMSUB");
                                            
                                            // 创建Fnmsubs指令
                                            let fnmsub_instr = RV64Instr::BaseIntInstr(super::rv64_instr::BaseIntInstr::MulAdd(super::rv64_instr::MulAdd::Fnmsubs {
                                                rd: sub_rd.clone(),
                                                rs1: fneg_rs.clone(),
                                                rs2: sub_rs2.clone(),
                                                rs3: sub_rs2.clone(),
                                            }));
                                            
                                            // 替换最后一条指令（FNEG_S）
                                            if let Some(last_stmt) = self.stmts.last_mut() {
                                                if let Asm::Riscv { instr } = last_stmt {
                                                    *instr = fnmsub_instr;
                                                }
                                            }
                                            
                                            return; // 不添加新指令，因为已经合并了
                                        }
                                    },
                                    _ => {}
                                }
                            },
                            _ =>()
                        }
                    },
                }
            },
        }
        
        // 4. 除法优化（2的幂次和简单除数）
        // 启用安全的除法优化
        if self.enable_division_optimization {
            match &riscv_instr {
                RV64Instr::BaseIntInstr(super::rv64_instr::BaseIntInstr::Arithmetic(super::rv64_instr::Arithmetic::DIV { rd: div_rd, rs1: div_rs1, rs2: div_rs2 })) => {
                    // 检查rs2是否是常量且是2的幂次
                    println!("检测到DIV指令，尝试64位除法优化");
                    
                    // 尝试将64位除法优化为右移
                    if let Some(shift_instr) = self.try_optimize_division_to_shift_64(div_rd.clone(), div_rs1.clone(), div_rs2.clone()) {
                        println!("64位除法优化成功：DIV -> 右移");
                        return self.asm(shift_instr);
                    }
                },
                RV64Instr::BaseIntInstr(super::rv64_instr::BaseIntInstr::Arithmetic(super::rv64_instr::Arithmetic::DIVW { rd: div_rd, rs1: div_rs1, rs2: div_rs2 })) => {
                    // 检查rs2是否是常量且是2的幂次
                    println!("检测到DIVW指令，尝试32位除法优化");
                    
                    // 尝试将32位除法优化为右移
                    println!("调试：DIVW指令 - rd={:?}, rs1={:?}, rs2={:?}", div_rd, div_rs1, div_rs2);
                    if let Some(shift_instr) = self.try_optimize_division_to_shift(div_rd.clone(), div_rs1.clone(), div_rs2.clone()) {
                        println!("32位除法优化成功：DIVW -> 右移");
                        return self.asm(shift_instr);
                    }
                },
                RV64Instr::BaseIntInstr(super::rv64_instr::BaseIntInstr::Arithmetic(super::rv64_instr::Arithmetic::FDIVS { rd: div_rd, rs1: div_rs1, rs2: div_rs2 })) => {
                    // 检查rs2是否是常量且是2的幂次
                    println!("检测到FDIVS指令，尝试浮点除法优化");
                    
                    // 尝试将浮点除法优化为浮点乘法（乘以倒数）
                    if let Some(mul_instr) = self.try_optimize_fdiv_to_fmul(div_rd.clone(), div_rs1.clone(), div_rs2.clone()) {
                        println!("浮点除法优化成功：FDIVS -> FMULS");
                        return self.asm(mul_instr);
                    }
                },
                _ => {}
            }
        }
        
        self.stmts.push(Asm::Riscv { instr: riscv_instr })
    }

    /// generate asm that initialize the value 
    pub fn apply_value(&mut self,val:&Value) -> Result<()>{
        match val{
            Value::Array { value_map, dims: _, ele_ty:_ } => {
                // if array
                let mut offset_value_pairs = value_map.iter().collect_vec();
                offset_value_pairs.sort_by_key(|x| x.0);

                let mut cur = 0 ;
                let mut last_offset = 0;
                while cur < offset_value_pairs.len(){
                    let (&offset,value) = offset_value_pairs[cur];
                    if offset > last_offset{
                        self.zero((offset - last_offset - 1)*val.get_ele_size()?);
                    }
                    self.apply_value(value)?;
                    last_offset = offset;
                    cur +=1 ;
                }
                if last_offset < val.get_mem_len()?{
                    debug_info_red!("mem_len:{} last_offset:{}",val.get_mem_len()?, last_offset);
                    self.zero(val.get_mem_len()? - last_offset*val.get_ele_size()?)
                }
                Ok(())
            },
            _ => {
                match val.get_ele_size()?{
                    8 => { self.double(Imm::new_literal(val.to_symidx().unwrap_or(SymIdx::new_verbose(0, "0".to_string(), None)).as_rc())) }
                    4 => { self.word(Imm::new_literal(val.to_symidx().unwrap_or(SymIdx::new_verbose(0, "0".to_string(), None)).as_rc())) }
                    2 => { self.half(Imm::new_literal(val.to_symidx().unwrap_or(SymIdx::new_verbose(0, "0".to_string(), None)).as_rc())) }
                    1 => { self.byte(Imm::new_literal(val.to_symidx().unwrap_or(SymIdx::new_verbose(0, "0".to_string(), None)).as_rc())) }
                    _ => { return Err(anyhow!("unexpected ele size")) }
                }
                Ok(())
            }
        }
    }
    
    /// 尝试将除法优化为右移指令
    /// 如果rs2是常量且是2的幂次，则返回对应的右移指令
    fn try_optimize_division_to_shift(&self, rd: Register, rs1: Register, rs2: Register) -> Option<RV64Instr> {
        // 检查rs2是否是常量且是2的幂次
        println!("调试：检查寄存器 {:?} 的常量值", rs2);
        if let Some(divisor_value) = self.get_register_constant_value(&rs2) {
            println!("调试：寄存器 {:?} 对应常量值 {}", rs2, divisor_value);
            if let Some(log2_value) = self.calculate_log2_if_power_of_two(divisor_value) {
                // 先打印优化信息
                println!("除法优化：{:?} / {} -> 右移 {} 位", rs1, divisor_value, log2_value);
                
                // 创建右移指令
                let shift_amount = log2_value as u32;
                let srliw_instr = super::rv64_instr::Shifts::Srliw {
                    rd: rd,
                    rs1: rs1,
                    shamt: super::rv64_instr::Imm::from_offset(shift_amount as isize)
                };
                
                return Some(RV64Instr::BaseIntInstr(BaseIntInstr::Shifts(srliw_instr)));
            }
        }
        
        None
    }
    
    /// 计算log2，如果输入是2的幂次则返回结果
    fn calculate_log2_if_power_of_two(&self, value: isize) -> Option<isize> {
        if value <= 0 {
            return None;
        }
        
        let value_abs = value.abs() as usize;
        if value_abs.is_power_of_two() {
            let mut log2 = 0;
            let mut temp = value_abs;
            while temp > 1 {
                temp >>= 1;
                log2 += 1;
            }
            Some(log2)
        } else {
            None
        }
    }
    
    /// 获取寄存器的常量值
    /// 从符号表中查找寄存器对应的真实常量值
    /// 
    /// 测试用例：
    /// - divw rd, rs1, t0  (t0=8)  -> srliw rd, rs1, 3
    /// - divw rd, rs1, t1  (t1=16) -> srliw rd, rs1, 4
    /// - divw rd, rs1, t2  (t2=32) -> srliw rd, rs1, 5
    /// - div rd, rs1, s0   (s0=2)  -> srli rd, rs1, 1
    /// - div rd, rs1, a0   (a0=1)  -> 不优化（除数为1）
    fn get_register_constant_value(&self, reg: &Register) -> Option<isize> {
        println!("开始查找寄存器 {:?} 的常量值", reg);
        
        // 优先从符号表获取真实常量值
        if let Some(symtab_rc) = &self.symtab {
            let symtab = symtab_rc.borrow();
            if let Some(value) = self.get_register_constant_value_from_symtab(reg, &symtab) {
                // 严格检查：只允许2的幂次和常见的简单除数
                if self.is_safe_divisor_for_optimization(value) {
                    println!("从符号表找到寄存器 {:?} 的安全常量值: {}", reg, value);
                    return Some(value);
                } else {
                    println!("从符号表找到寄存器 {:?} 的常量值 {}，但不安全，跳过优化", reg, value);
                    return None;
                }
            }
        }
        
        // 如果符号表中没有找到，尝试从指令历史中推断常量值
        if let Some(value) = self.infer_constant_from_instruction_history(reg) {
            // 严格检查：只允许2的幂次和常见的简单除数
            if self.is_safe_divisor_for_optimization(value) {
                println!("从指令历史找到寄存器 {:?} 的安全常量值: {}", reg, value);
                return Some(value);
            } else {
                println!("从指令历史找到寄存器 {:?} 的常量值 {}，但不安全，跳过优化", reg, value);
                return None;
            }
        }
        
        // 对于参数寄存器，不进行硬编码推断，避免错误
        match reg {
            Register::Arg { .. } => {
                println!("寄存器 {:?} 是参数寄存器，不进行硬编码推断", reg);
                None
            },
            _ => {
                let value = self.infer_constant_generically(reg);
                if let Some(v) = value {
                    // 严格检查：只允许2的幂次和常见的简单除数
                    if self.is_safe_divisor_for_optimization(v) {
                        println!("从通用推断找到寄存器 {:?} 的安全常量值: {}", reg, v);
                        return Some(v);
                    } else {
                        println!("从通用推断找到寄存器 {:?} 的常量值 {}，但不安全，跳过优化", reg, v);
                        return None;
                    }
                } else {
                    println!("未找到寄存器 {:?} 的常量值", reg);
                }
                None
            }
        }
    }
    
    /// 检查是否是安全的除数（可以安全地进行优化）
    /// 只允许2的幂次和少数经过验证的简单除数
    fn is_safe_divisor_for_optimization(&self, value: isize) -> bool {
        if value <= 0 {
            return false; // 除数为0或负数不安全
        }
        
        // 只允许2的幂次（最安全）
        if self.is_power_of_two(value) {
            return true;
        }
        
        // 暂时禁用非2幂次的简单除数优化
        // 这些值虽然数学上可以优化，但实现复杂且收益有限
        false
    }
    
    /// 从指令历史中推断寄存器的常量值
    /// 通过分析之前的指令来推断寄存器可能包含的常量值
    fn infer_constant_from_instruction_history(&self, reg: &Register) -> Option<isize> {
        // 遍历最近的几条指令，查找加载常量的指令
        let history_size = 200; // 增加搜索范围
        let start_idx = if self.stmts.len() > history_size {
            self.stmts.len() - history_size
        } else {
            0
        };
        
        // 从最近的指令开始向前搜索，优先找到最近的赋值
        for i in (start_idx..self.stmts.len()).rev() {
            if let Asm::Riscv { instr } = &self.stmts[i] {
                // 1. 首先尝试直接常量加载指令
                if self.is_constant_loading_instruction(instr, reg) {
                    if let Some(value) = self.extract_constant_from_instruction(instr, reg) {
                        // 检查这个值是否合理（避免0值或其他明显错误的值）
                        if self.is_reasonable_constant_value(value, reg) {
                            println!("从指令历史中找到寄存器 {:?} 的常量值: {}", reg, value);
                            return Some(value);
                        } else {
                            println!("忽略不合理的常量值: {} (寄存器: {:?})", value, reg);
                        }
                    }
                }
                
                // 2. 检查是否是函数调用的返回值
                if self.is_function_call_return(instr, reg) {
                    println!("检测到寄存器 {:?} 可能是函数调用返回值", reg);
                    // 对于函数调用返回值，我们无法静态推断其值
                    // 但可以尝试从调用上下文推断
                    return self.infer_value_from_function_context(reg, i);
                }
                
                // 3. 检查是否是内存加载
                if self.is_memory_load_to_register(instr, reg) {
                    println!("检测到寄存器 {:?} 从内存加载", reg);
                    // 尝试追踪内存中的值
                    return self.infer_value_from_memory_load(instr, reg, i);
                }
            }
        }
        
        // 如果没有找到直接匹配，尝试分析寄存器之间的数据流
        self.analyze_register_data_flow(reg)
    }
    
    /// 检查指令是否是函数调用的返回值
    fn is_function_call_return(&self, instr: &RV64Instr, target_reg: &Register) -> bool {
        // 检查是否是 call 指令后的返回值
        // 在RISC-V中，函数返回值通常在 a0 寄存器中
        match instr {
            RV64Instr::PseudoInstr(super::rv64_instr::PseudoInstr::Call { .. }) => {
                // 如果目标寄存器是 a0，可能是函数返回值
                matches!(target_reg, Register::Arg { reg_idx: 0 })
            },
            _ => false
        }
    }
    
    /// 从函数调用上下文推断值
    fn infer_value_from_function_context(&self, reg: &Register, current_idx: usize) -> Option<isize> {
        // 对于函数调用返回值，我们无法静态推断其值
        // 但可以根据调用函数的类型进行一些推断
        
        // 向前搜索最近的函数调用
        for i in (0..current_idx).rev() {
            if let Asm::Riscv { instr } = &self.stmts[i] {
                if let RV64Instr::PseudoInstr(super::rv64_instr::PseudoInstr::Call { offset }) = instr {
                    // 检查是否是 quick_read 函数调用
                    if self.is_quick_read_call(offset) {
                        println!("检测到 quick_read 函数调用，返回值可能来自用户输入");
                        // 对于 quick_read，我们无法静态推断其值
                        return None;
                    }
                }
            }
        }
        
        println!("无法静态推断函数调用返回值，跳过优化");
        None
    }
    
    /// 检查是否是 quick_read 函数调用
    fn is_quick_read_call(&self, offset: &super::rv64_instr::Imm) -> bool {
        // 这里需要根据实际的符号表来判断
        // 暂时返回 false
        false
    }
    
    /// 检查指令是否是内存加载到寄存器
    fn is_memory_load_to_register(&self, instr: &RV64Instr, target_reg: &Register) -> bool {
        match instr {
            RV64Instr::BaseIntInstr(super::rv64_instr::BaseIntInstr::Loads(
                super::rv64_instr::Loads::Lw { rd, .. }
            )) => {
                rd == target_reg
            },
            _ => false
        }
    }
    
    /// 从内存加载推断值
    fn infer_value_from_memory_load(&self, instr: &RV64Instr, reg: &Register, current_idx: usize) -> Option<isize> {
        // 尝试追踪内存中存储的值
        // 这需要分析之前的存储指令
        println!("无法静态推断内存加载的值，跳过优化");
        None
    }
    
    /// 检查常量值是否合理
    fn is_reasonable_constant_value(&self, value: isize, reg: &Register) -> bool {
        // 对于除法优化，我们主要关心2的幂次值
        if value <= 0 {
            return false; // 除数为0或负数不合理
        }
        
        // 检查是否是2的幂次
        if self.is_power_of_two(value) {
            return true;
        }
        
        // 对于其他值，可以根据寄存器类型进行判断
        match reg {
            Register::Arg { reg_idx } => {
                // 参数寄存器通常包含用户输入或函数返回值
                // 允许较大的值
                value > 0 && value <= 10000
            },
            _ => {
                // 其他寄存器允许合理的常量值
                value > 0 && value <= 1000
            }
        }
    }
    
    /// 检查值是否是2的幂次
    fn is_power_of_two(&self, value: isize) -> bool {
        if value <= 0 {
            return false;
        }
        
        let value_abs = value.abs() as u32;
        value_abs.is_power_of_two()
    }
    
    /// 检查指令是否是真正的常量加载指令
    fn is_constant_loading_instruction(&self, instr: &RV64Instr, target_reg: &Register) -> bool {
        match instr {
            RV64Instr::PseudoInstr(super::rv64_instr::PseudoInstr::Li { rd, .. }) => {
                let matches = rd == target_reg;
                if matches {
                    println!("找到匹配的 li 指令: {:?} -> {:?}", instr, target_reg);
                }
                matches
            },
            RV64Instr::BaseIntInstr(super::rv64_instr::BaseIntInstr::Arithmetic(
                super::rv64_instr::Arithmetic::ADDI { rd, rs1, .. }
            )) => {
                // 只匹配 addi rd, zero, imm 形式的指令
                let matches = rd == target_reg && rs1 == &Register::Zero {};
                if matches {
                    println!("找到匹配的 addi 指令: {:?} -> {:?}", instr, target_reg);
                }
                matches
            },
            _ => false
        }
    }
    
    /// 分析寄存器之间的数据流
    /// 通过追踪寄存器之间的移动和计算来推断值
    fn analyze_register_data_flow(&self, target_reg: &Register) -> Option<isize> {
        // 实现更复杂的数据流分析
        let history_size = 50; // 增加搜索范围
        let start_idx = if self.stmts.len() > history_size {
            self.stmts.len() - history_size
        } else {
            0
        };
        
        // 1. 查找对该寄存器的直接赋值
        for i in start_idx..self.stmts.len() {
            if let Asm::Riscv { instr } = &self.stmts[i] {
                if let Some(value) = self.extract_constant_from_instruction(instr, target_reg) {
                    return Some(value);
                }
            }
        }
        
        // 2. 查找寄存器之间的移动操作
        for i in start_idx..self.stmts.len() {
            if let Asm::Riscv { instr } = &self.stmts[i] {
                if let Some(value) = self.track_register_moves(instr, target_reg, i) {
                    return Some(value);
                }
            }
        }
        
        // 3. 查找算术运算的结果
        for i in start_idx..self.stmts.len() {
            if let Asm::Riscv { instr } = &self.stmts[i] {
                if let Some(value) = self.track_arithmetic_results(instr, target_reg, i) {
                    return Some(value);
                }
            }
        }
        
        None
    }
    
    /// 追踪寄存器移动操作
    fn track_register_moves(&self, instr: &RV64Instr, target_reg: &Register, current_idx: usize) -> Option<isize> {
        match instr {
            RV64Instr::PseudoInstr(super::rv64_instr::PseudoInstr::Mv { rd, rs }) => {
                if rd == target_reg {
                    // 如果目标寄存器是我们要查找的寄存器，尝试从源寄存器获取值
                    return self.get_register_constant_value(rs);
                }
            },
            _ => {}
        }
        
        None
    }
    
    /// 追踪算术运算结果
    fn track_arithmetic_results(&self, instr: &RV64Instr, target_reg: &Register, current_idx: usize) -> Option<isize> {
        match instr {
            RV64Instr::BaseIntInstr(super::rv64_instr::BaseIntInstr::Arithmetic(
                super::rv64_instr::Arithmetic::ADD { rd, rs1, rs2 }
            )) => {
                if rd == target_reg {
                    // 尝试计算加法结果
                    if let (Some(val1), Some(val2)) = (
                        self.get_register_constant_value(rs1),
                        self.get_register_constant_value(rs2)
                    ) {
                        return Some(val1 + val2);
                    }
                }
            },
            RV64Instr::BaseIntInstr(super::rv64_instr::BaseIntInstr::Arithmetic(
                super::rv64_instr::Arithmetic::SUB { rd, rs1, rs2 }
            )) => {
                if rd == target_reg {
                    // 尝试计算减法结果
                    if let (Some(val1), Some(val2)) = (
                        self.get_register_constant_value(rs1),
                        self.get_register_constant_value(rs2)
                    ) {
                        return Some(val1 - val2);
                    }
                }
            },
            RV64Instr::BaseIntInstr(super::rv64_instr::BaseIntInstr::Arithmetic(
                super::rv64_instr::Arithmetic::MUL { rd, rs1, rs2 }
            )) => {
                if rd == target_reg {
                    // 尝试计算乘法结果
                    if let (Some(val1), Some(val2)) = (
                        self.get_register_constant_value(rs1),
                        self.get_register_constant_value(rs2)
                    ) {
                        return Some(val1 * val2);
                    }
                }
            },
            _ => {}
        }
        
        None
    }
    
    /// 从单条指令中提取常量值
    fn extract_constant_from_instruction(&self, instr: &RV64Instr, target_reg: &Register) -> Option<isize> {
        match instr {
            RV64Instr::PseudoInstr(super::rv64_instr::PseudoInstr::Li { rd, imm }) => {
                if rd == target_reg {
                    // 从立即数中提取常量值
                    let value = self.extract_constant_from_imm(imm);
                    if let Some(v) = value {
                        println!("从指令 {:?} 中提取到寄存器 {:?} 的常量值: {}", instr, target_reg, v);
                    }
                    return value;
                }
            },
            RV64Instr::BaseIntInstr(super::rv64_instr::BaseIntInstr::Arithmetic(
                super::rv64_instr::Arithmetic::ADDI { rd, rs1, imm }
            )) => {
                // 只匹配 addi rd, zero, imm 形式的指令（等价于 li rd, imm）
                if rd == target_reg && rs1 == &Register::Zero {} {
                    let value = self.extract_constant_from_imm(imm);
                    if let Some(v) = value {
                        println!("从指令 {:?} 中提取到寄存器 {:?} 的常量值: {}", instr, target_reg, v);
                    }
                    return value;
                }
            },
            RV64Instr::BaseIntInstr(super::rv64_instr::BaseIntInstr::Arithmetic(
                super::rv64_instr::Arithmetic::LUI { rd, imm }
            )) => {
                if rd == target_reg {
                    // lui rd, imm 加载高位立即数
                    let value = self.extract_constant_from_imm(imm);
                    if let Some(v) = value {
                        println!("从指令 {:?} 中提取到寄存器 {:?} 的常量值: {}", instr, target_reg, v);
                    }
                    return value;
                }
            },
            RV64Instr::BaseIntInstr(super::rv64_instr::BaseIntInstr::Arithmetic(
                super::rv64_instr::Arithmetic::AUIPC { rd, imm }
            )) => {
                if rd == target_reg {
                    // auipc rd, imm 加载PC相对地址
                    let value = self.extract_constant_from_imm(imm);
                    if let Some(v) = value {
                        println!("从指令 {:?} 中提取到寄存器 {:?} 的常量值: {}", instr, target_reg, v);
                    }
                    return value;
                }
            },
            _ => {}
        }
        
        None
    }
    
    /// 从立即数中提取常量值
    fn extract_constant_from_imm(&self, imm: &super::rv64_instr::Imm) -> Option<isize> {
        match imm {
            super::rv64_instr::Imm::Literal { symidx } => {
                // 尝试从符号索引中提取数值
                if let Some(value) = self.extract_value_from_symidx(symidx) {
                    return Some(value);
                }
            },
            _ => {}
        }
        
        None
    }
    
    /// 从符号索引中提取数值
    fn extract_value_from_symidx(&self, symidx: &super::symtab::RcSymIdx) -> Option<isize> {
        // 尝试将符号索引转换为数值
        if let std::result::Result::Ok(value) = super::field::Value::from_symidx(&symidx.as_ref_borrow()) {
            if let std::result::Result::Ok(int_value) = value.as_i32() {
                println!("从符号索引 {:?} 提取到数值: {}", symidx, int_value);
                return Some(int_value as isize);
            }
        }
        

        
        None
    }
    
    /// 尝试将64位除法优化为右移指令
    /// 如果rs2是常量且是2的幂次，则返回对应的右移指令
    fn try_optimize_division_to_shift_64(&self, rd: Register, rs1: Register, rs2: Register) -> Option<RV64Instr> {
        // 检查rs2是否是常量且是2的幂次
        if let Some(divisor_value) = self.get_register_constant_value(&rs2) {
            if let Some(log2_value) = self.calculate_log2_if_power_of_two(divisor_value) {
                // 先打印优化信息
                println!("64位除法优化：{:?} / {} -> 右移 {} 位", rs1, divisor_value, log2_value);
                
                // 创建64位右移指令
                let shift_amount = log2_value as u32;
                let srli_instr = super::rv64_instr::Shifts::Srli {
                    rd: rd,
                    rs1: rs1,
                    shamt: super::rv64_instr::Imm::from_offset(shift_amount as isize)
                };
                
                return Some(RV64Instr::BaseIntInstr(BaseIntInstr::Shifts(srli_instr)));
            }
        }
        
        None
    }
    
    /// 通用常量推断
    /// 基于寄存器类型和上下文进行智能推断
    fn infer_constant_generically(&self, reg: &Register) -> Option<isize> {
        // 移除硬编码的常量推断，避免错误的优化
        // 只依赖指令历史分析和符号表查找
        match reg {
            Register::Arg { reg_idx } => {
                // 参数寄存器可能包含函数参数或常量
                // 由于参数寄存器的值通常来自函数调用或用户输入，我们不应该进行硬编码推断
                // 而是应该依赖指令历史分析或符号表查找
                None
            },
            _ => {
                // 对于其他类型的寄存器，不进行硬编码推断
                // 避免错误的常量推断导致语义修改
                None
            }
        }
    }
    
    /// 尝试优化非2的幂次除法
    /// 使用更通用的算法来处理各种除数
    fn try_optimize_non_power_of_two_division(&self, rd: Register, rs1: Register, divisor: isize) -> Option<RV64Instr> {
        if divisor <= 0 {
            return None;
        }
        
        // 对于常见的非2幂次除数，使用乘法优化
        let divisor_abs = divisor.abs() as u32;
        
        // 计算乘法因子和移位量
        if let Some((multiplier, shift)) = self.calculate_division_multiplier(divisor_abs) {
            println!("优化除法：{:?} / {} -> 乘法优化 (乘数={}, 移位={})", rs1, divisor, multiplier, shift);
            
            // 创建乘法优化序列
            // 注意：这里需要处理符号问题
            if divisor > 0 {
                // 正数除法：直接使用乘法优化
                // 生成：rd = (rs1 * multiplier) >> shift
                // 由于需要多条指令，我们暂时只返回乘法指令
                // 实际实现需要完整的指令序列
                let mul_instr = super::rv64_instr::Arithmetic::MUL {
                    rd: rd.clone(),
                    rs1: rs1,
                    rs2: Register::Temp { reg_idx: 0 } // 需要先加载multiplier到t0
                };
                
                // 注意：这里只是示例，实际需要：
                // 1. li t0, multiplier
                // 2. mul rd, rs1, t0  
                // 3. srai rd, rd, shift
                println!("警告：非2幂次除法优化需要多条指令，暂时返回None");
                None
            } else {
                // 负数除法：需要特殊处理
                println!("警告：负数除法优化暂时禁用");
                None
            }
        } else {
            None
        }
    }
    
    /// 计算除法的乘法因子
    /// 使用数学方法计算最优的乘数和移位量
    fn calculate_division_multiplier(&self, divisor: u32) -> Option<(u32, u32)> {
        if divisor == 0 {
            return None;
        }
        
        // 对于常见的除数，使用预计算的优化参数
        match divisor {
            3 => Some((0x55555556, 32)),   // x/3 ≈ (x * 0x55555556) >> 32
            5 => Some((0x33333333, 32)),   // x/5 ≈ (x * 0x33333333) >> 32
            7 => Some((0x24924925, 32)),   // x/7 ≈ (x * 0x24924925) >> 32
            9 => Some((0x1c71c71d, 32)),  // x/9 ≈ (x * 0x1c71c71d) >> 32
            10 => Some((0x1999999a, 32)), // x/10 ≈ (x * 0x1999999a) >> 32
            11 => Some((0x1745d175, 32)), // x/11 ≈ (x * 0x1745d175) >> 32
            13 => Some((0x13b13b14, 32)), // x/13 ≈ (x * 0x13b13b14) >> 32
            15 => Some((0x11111111, 32)), // x/15 ≈ (x * 0x11111111) >> 32
            _ => {
                // 对于其他除数，使用通用算法计算
                self.calculate_general_division_multiplier(divisor)
            }
        }
    }
    
    /// 计算通用除法的乘法因子
    /// 使用数学方法计算最优的乘数和移位量
    fn calculate_general_division_multiplier(&self, divisor: u32) -> Option<(u32, u32)> {
        if divisor == 0 || divisor == 1 {
            return None;
        }
        
        // 计算需要的精度
        let precision = 32;
        let max_value = (1u64 << precision) - 1;
        
        // 计算乘法因子：m = (2^precision) / divisor
        let multiplier = (max_value / divisor as u64 + 1) as u32;
        
        // 计算移位量：通常为precision
        let shift = precision;
        
        Some((multiplier, shift))
    }
    
    /// 尝试将浮点除法优化为浮点乘法（乘以倒数）
    /// 使用更通用的方法处理浮点除法优化
    fn try_optimize_fdiv_to_fmul(&self, rd: Register, rs1: Register, rs2: Register) -> Option<RV64Instr> {
        // 检查rs2是否是常量
        if let Some(divisor_value) = self.get_register_constant_value(&rs2) {
            if divisor_value == 0 {
                // 除零错误，不进行优化
                return None;
            }
            
            // 只对2的幂次进行浮点除法优化，避免精度问题
            if let Some(log2_value) = self.calculate_log2_if_power_of_two(divisor_value) {
                // 对于2的幂次，可以优化为乘法
                println!("浮点除法优化：{:?} / {} -> 乘法优化", rs1, divisor_value);
                
                // 计算倒数：1 / (2^log2) = 2^(-log2)
                let reciprocal = 1.0 / (1 << log2_value) as f32;
                
                // 创建浮点乘法指令
                // 注意：这里需要加载倒数值到寄存器
                // 由于当前架构限制，暂时不进行优化
                println!("警告：浮点除法优化暂时禁用，避免程序错误");
                None
            } else {
                // 对于非2的幂次除数，暂时不进行优化
                println!("警告：非2幂次浮点除法优化暂时禁用，避免精度问题");
                None
            }
        } else {
            None
        }
    }
    
    /// 尝试优化非2幂次的浮点除法
    fn try_optimize_non_power_of_two_fdiv(&self, rd: Register, rs1: Register, divisor: isize) -> Option<RV64Instr> {
        // 对于浮点除法，可以尝试使用牛顿迭代法或其他数值方法
        // 但由于精度和性能考虑，暂时不进行优化
        println!("浮点除法优化：{:?} / {} -> 暂不优化", rs1, divisor);
        None
    }
    
    /// 从符号表中获取寄存器的真实常量值
    /// 充分利用symtab.rs中的信息进行智能推断
    fn get_register_constant_value_from_symtab(&self, reg: &Register, symtab: &super::symtab::SymTab) -> Option<isize> {
        // 1. 使用智能查找功能
        if let Some(value) = self.smart_find_symbol_in_symtab(reg, symtab) {
            return Some(value);
        }
        
        // 2. 尝试从指令历史中查找该寄存器的赋值
        if let Some(value) = self.find_register_value_in_instruction_history(reg, symtab) {
            return Some(value);
        }
        
        // 3. 尝试从符号表的迭代器中查找匹配的符号
        for (symidx, symbol) in symtab.iter() {
            // 检查符号名是否与寄存器匹配
            if self.is_symbol_matching_register(symidx, reg) {
                // 检查是否是常量
                if symidx.is_literal() {
                    if let std::result::Result::Ok(value) = super::field::Value::from_symidx(symidx) {
                        if let std::result::Result::Ok(int_value) = value.as_i32() {
                            return Some(int_value as isize);
                        }
                    }
                }
                
                // 检查符号的字段
                if let Some(constant_value) = self.extract_constant_from_symbol(symbol) {
                    return Some(constant_value);
                }
            }
        }
        
        None
    }
    
    /// 检查符号是否与寄存器匹配
    fn is_symbol_matching_register(&self, symidx: &super::symtab::SymIdx, reg: &Register) -> bool {
        let symbol_name = &symidx.symbol_name;
        
        match reg {
            Register::Temp { reg_idx } => {
                symbol_name == &format!("t{}", reg_idx) ||
                symbol_name == &format!("temp_{}", reg_idx) ||
                symbol_name == &format!("tmp{}", reg_idx) ||
                symbol_name == &format!("t{}_ssa", reg_idx)
            },
            Register::Saved { reg_idx } => {
                symbol_name == &format!("s{}", reg_idx) ||
                symbol_name == &format!("saved_{}", reg_idx) ||
                symbol_name == &format!("s{}_ssa", reg_idx)
            },
            Register::Arg { reg_idx } => {
                symbol_name == &format!("a{}", reg_idx) ||
                symbol_name == &format!("arg{}", reg_idx) ||
                symbol_name == &format!("param{}", reg_idx) ||
                symbol_name == &format!("a{}_ssa", reg_idx)
            },
            Register::FArg { reg_idx } => {
                symbol_name == &format!("fa{}", reg_idx) ||
                symbol_name == &format!("farg{}", reg_idx) ||
                symbol_name == &format!("fa{}_ssa", reg_idx)
            },
            Register::FSaved { reg_idx } => {
                symbol_name == &format!("f{}", reg_idx) ||
                symbol_name == &format!("fsaved{}", reg_idx) ||
                symbol_name == &format!("f{}_ssa", reg_idx)
            },
            _ => false
        }
    }
    
    /// 获取寄存器可能对应的符号名列表
    fn get_possible_symbol_names(&self, reg: &Register) -> Vec<String> {
        let mut names = Vec::new();
        
        match reg {
            Register::Temp { reg_idx } => {
                names.push(format!("t{}", reg_idx));
                names.push(format!("temp_{}", reg_idx));
                names.push(format!("tmp{}", reg_idx));
                names.push(format!("t{}_ssa", reg_idx));
            },
            Register::Saved { reg_idx } => {
                names.push(format!("s{}", reg_idx));
                names.push(format!("saved_{}", reg_idx));
                names.push(format!("s{}_ssa", reg_idx));
            },
            Register::Arg { reg_idx } => {
                names.push(format!("a{}", reg_idx));
                names.push(format!("arg{}", reg_idx));
                names.push(format!("param{}", reg_idx));
                names.push(format!("a{}_ssa", reg_idx));
            },
            Register::FArg { reg_idx } => {
                names.push(format!("fa{}", reg_idx));
                names.push(format!("farg{}", reg_idx));
                names.push(format!("fa{}_ssa", reg_idx));
            },
            Register::FSaved { reg_idx } => {
                names.push(format!("f{}", reg_idx));
                names.push(format!("fsaved{}", reg_idx));
                names.push(format!("f{}_ssa", reg_idx));
            },
            _ => {}
        }
        
        names
    }
    
    /// 智能查找符号表中的符号
    fn smart_find_symbol_in_symtab(&self, reg: &Register, symtab: &super::symtab::SymTab) -> Option<isize> {
        let possible_names = self.get_possible_symbol_names(reg);
        
        // 尝试不同的作用域
        for scope in 0..10 { // 尝试前10个作用域
            for reg_name in &possible_names {
                let symidx = super::symtab::SymIdx::new(scope, reg_name.clone());
                
                if symtab.has_symbol(&symidx) {
                    if let std::result::Result::Ok(symbol) = symtab.get(&symidx) {
                        // 检查是否是常量
                        if symidx.is_literal() {
                            if let std::result::Result::Ok(value) = super::field::Value::from_symidx(&symidx) {
                                if let std::result::Result::Ok(int_value) = value.as_i32() {
                                    return Some(int_value as isize);
                                }
                            }
                        }
                        
                        // 检查符号的字段
                        if let Some(constant_value) = self.extract_constant_from_symbol(symbol) {
                            return Some(constant_value);
                        }
                    }
                }
            }
        }
        
        None
    }
    
    /// 从符号中提取常量值
    fn extract_constant_from_symbol(&self, symbol: &super::symbol::Symbol) -> Option<isize> {
        // 检查符号的字段中是否有常量值
        // 这里可以根据Symbol的具体结构来实现
        None
    }
    
    /// 从指令历史中查找寄存器的值
    fn find_register_value_in_instruction_history(&self, reg: &Register, symtab: &super::symtab::SymTab) -> Option<isize> {
        // 遍历指令历史，查找对该寄存器的赋值
        let history_size = 30; // 增加搜索范围
        let start_idx = if self.stmts.len() > history_size {
            self.stmts.len() - history_size
        } else {
            0
        };
        
        for i in start_idx..self.stmts.len() {
            if let Asm::Riscv { instr } = &self.stmts[i] {
                // 检查是否是加载指令
                if let Some(value) = self.extract_constant_from_instruction(instr, reg) {
                    return Some(value);
                }
                
                // 检查是否是移动指令（从另一个寄存器移动值）
                if let Some(value) = self.extract_value_from_move_instruction(instr, reg, symtab) {
                    return Some(value);
                }
            }
        }
        
        None
    }
    
    /// 从移动指令中提取值
    fn extract_value_from_move_instruction(&self, instr: &RV64Instr, target_reg: &Register, symtab: &super::symtab::SymTab) -> Option<isize> {
        match instr {
            RV64Instr::PseudoInstr(super::rv64_instr::PseudoInstr::Mv { rd, rs }) => {
                if rd == target_reg {
                    // 如果目标寄存器是我们要查找的寄存器，尝试从源寄存器获取值
                    return self.get_register_constant_value_from_symtab(rs, symtab);
                }
            },
            _ => {}
        }
        
        None
    }
    
    /// 将寄存器转换为符号名
    fn register_to_symbol_name(&self, reg: &Register) -> Option<String> {
        match reg {
            Register::Temp { reg_idx } => Some(format!("t{}", reg_idx)),
            Register::Saved { reg_idx } => Some(format!("s{}", reg_idx)),
            Register::Arg { reg_idx } => Some(format!("a{}", reg_idx)),
            Register::FArg { reg_idx } => Some(format!("fa{}", reg_idx)),
            Register::FSaved { reg_idx } => Some(format!("f{}", reg_idx)),
            _ => None
        }
    }
    
}

/// a section contains several attributes and then asm instrs
pub struct AsmSection{
    pub sect_name:String,
    pub stmts:Vec<Asm>,
    pub symtab: Option<std::rc::Rc<std::cell::RefCell<super::symtab::SymTab>>>,
    pub enable_division_optimization: bool
}

/// all kinds of Attr 
pub enum Asm{
    Attr{attr:AsmAttr},
    Riscv{
        instr:RV64Instr
    }
}
impl Debug for Asm{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Asm::Attr { attr } => {
                match attr{
                    AsmAttr::Align { align } => {
                        writeln!(f,"    .align {}",align)
                    },
                    AsmAttr::Global { label} => {
                        writeln!(f,"    .globl {:?}",label)
                    },
                    AsmAttr::Data {  } => {
                        writeln!(f,"    .data")
                    },
                    AsmAttr::Text {  } => {
                        writeln!(f,"    .text")
                    },
                    AsmAttr::Double { imm } => {
                        writeln!(f,"    .double {:?}",imm)
                    },
                    AsmAttr::Word { imm } => {
                        writeln!(f,"    .word {:?}",imm)
                    },
                    AsmAttr::Half { imm } => {
                        writeln!(f,"    .half {:?}",imm)
                    },
                    AsmAttr::Byte { imm } => {
                        writeln!(f,"    .byte {:?}",imm)
                    },
                    AsmAttr::Zero { len } => {
                        writeln!(f,"    .zero {:?}",len)
                    },
                    AsmAttr::Label { imm } => {
                        writeln!(f,"{:?}:",imm)
                    },
                    AsmAttr::DataType { attr_ty, imm: symidx } => {
                        writeln!(f,"    .type {:?},{:?}",symidx, attr_ty)
                    },
                    AsmAttr::Annotation { annotation } => {
                        write!(f,"                   {}\n",annotation )
                    },
                }
            },
            Asm::Riscv { instr } => {
                writeln!(f,"    {:?}", instr)
            },
        }
    }
}

#[derive(EnumIs)]
pub enum AsmAttr{
    Annotation{
        annotation:String
    },
    Align{
        align:usize
    },
    Global{
        label:Imm
    },
    Data{ },Text{},
    Double{
        imm:Imm
    },
    Word{
        imm:Imm
    },
    Half{
        imm:Imm
    },
    Byte{
        imm:Imm
    },
    Zero{
        len:usize
    },
    Label{
        imm:Imm
    },
    DataType{
        attr_ty:DataType,
        imm:Imm
    }
}
pub enum DataType{
    Object, 
    Function
}
impl Debug for DataType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Object => write!(f, "@object"),
            Self::Function => write!(f, "@function"),
        }
    }
}
impl Into<Asm> for AsmAttr{
    fn into(self) -> Asm {
        Asm::Attr { attr: self }
    }
}
impl Into<Asm> for RV64Instr{
    fn into(self) -> Asm {
        Asm::Riscv { instr: self } 
    }
}
impl AsmSection{
    pub fn dump(&self, enable_annotation:bool) -> String{
        let mut s = String::new();
        let mut i =0;
        while i < self.stmts.len(){
            let asm_attr = self.stmts.get(i).unwrap();
            if !enable_annotation{
                match asm_attr{
                    Asm::Attr { attr:AsmAttr::Annotation { annotation } } => {
                        
                    },
                    _ => {
                        s += format!("{:?}",asm_attr).as_str();
                    }
                }
            }else{
                match asm_attr{
                    Asm::Attr { attr: AsmAttr::Annotation { annotation } } => {
                        let attr = self.stmts.get(i+1);
                        if !matches!(attr,Some(Asm::Attr { attr: AsmAttr::Annotation { annotation } })) && annotation.ends_with("\n"){
                            // next is not annotation so you should add \n
                            s += format!("              # {:?}\n",asm_attr).as_str();
                        }else {
                            s += format!("              # {:?}",asm_attr).as_str();
                        }
                    },
                    _ => {
                        s += format!("{:?}",asm_attr).as_str();
                    }
                }
            }
            i += 1;
        }
        format!(".section {}\n{}",self.sect_name,s)
    }
}