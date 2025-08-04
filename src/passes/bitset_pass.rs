use crate::toolkit::context::NhwcCtx;
use anyhow::Result;
use crate::toolkit::pass_manager::Pass;
use regex::Regex;

pub struct BitsetPass {
    debug: bool,
}

impl BitsetPass {
    pub fn new(debug: bool) -> Self { 
        Self { debug } 
    }

    /// 优化移位表达式
    fn optimize_shift_expressions(&mut self, ctx: &mut NhwcCtx) -> Result<()> {
        if self.debug {
            println!("BitsetPass: 优化移位表达式");
        }
        
        let mut new_code = ctx.code.clone();
        
        // 左移优化: x << n -> x * (2^n)
        let shift_left_pattern = Regex::new(r"(\w+)\s*<<\s*(\d+)").unwrap();
        new_code = shift_left_pattern.replace_all(&new_code, |caps: &regex::Captures| {
            let var = &caps[1];
            let shift = caps[2].parse::<i32>().unwrap();
            if shift > 0 && shift <= 31 {
                let power = 1 << shift;
                let replacement = format!("{} * {}", var, power);
                if self.debug {
                    println!("BitsetPass: 优化左移 {} << {} -> {}", var, shift, replacement);
                }
                replacement
            } else {
                caps[0].to_string()
            }
        }).to_string();
        
        // 右移优化: x >> n -> x / (2^n)
        let shift_right_pattern = Regex::new(r"(\w+)\s*>>\s*(\d+)").unwrap();
        new_code = shift_right_pattern.replace_all(&new_code, |caps: &regex::Captures| {
            let var = &caps[1];
            let shift = caps[2].parse::<i32>().unwrap();
            if shift > 0 && shift <= 31 {
                let power = 1 << shift;
                let replacement = format!("{} / {}", var, power);
                if self.debug {
                    println!("BitsetPass: 优化右移 {} >> {} -> {}", var, shift, replacement);
                }
                replacement
            } else {
                caps[0].to_string()
            }
        }).to_string();
        
        ctx.code = new_code;
        Ok(())
    }

    /// 优化位与表达式
    fn optimize_and_expressions(&mut self, ctx: &mut NhwcCtx) -> Result<()> {
        if self.debug {
            println!("BitsetPass: 优化位与表达式");
        }
        
        let mut new_code = ctx.code.clone();
        
        // x & 0 -> 0
        let and_zero_pattern = Regex::new(r"(\w+)\s*&\s*0").unwrap();
        new_code = and_zero_pattern.replace_all(&new_code, "0").to_string();
        
        // x & x -> x (移除反向引用，因为Rust regex不支持)
        // let and_self_pattern = Regex::new(r"(\w+)\s*&\s*\1").unwrap();
        // new_code = and_self_pattern.replace_all(&new_code, "$1").to_string();
        
        // x & 0xFFFFFFFF -> x
        let and_all_pattern = Regex::new(r"(\w+)\s*&\s*0x[Ff]{8}").unwrap();
        new_code = and_all_pattern.replace_all(&new_code, "$1").to_string();
        
        ctx.code = new_code;
        Ok(())
    }

    /// 优化位或表达式
    fn optimize_or_expressions(&mut self, ctx: &mut NhwcCtx) -> Result<()> {
        if self.debug {
            println!("BitsetPass: 优化位或表达式");
        }
        
        let mut new_code = ctx.code.clone();
        
        // x | 0 -> x
        let or_zero_pattern = Regex::new(r"(\w+)\s*\|\s*0").unwrap();
        new_code = or_zero_pattern.replace_all(&new_code, "$1").to_string();
        
        // x | x -> x (移除反向引用，因为Rust regex不支持)
        // let or_self_pattern = Regex::new(r"(\w+)\s*\|\s*\1").unwrap();
        // new_code = or_self_pattern.replace_all(&new_code, "$1").to_string();
        
        // x | 0xFFFFFFFF -> 0xFFFFFFFF
        let or_all_pattern = Regex::new(r"(\w+)\s*\|\s*0x[Ff]{8}").unwrap();
        new_code = or_all_pattern.replace_all(&new_code, "0xFFFFFFFF").to_string();
        
        ctx.code = new_code;
        Ok(())
    }



    /// 优化逻辑表达式
    fn optimize_logical_expressions(&mut self, ctx: &mut NhwcCtx) -> Result<()> {
        if self.debug {
            println!("BitsetPass: 优化逻辑表达式");
        }
        
        let mut new_code = ctx.code.clone();
        
        // x && true -> x
        let and_true_pattern = Regex::new(r"(\w+)\s*&&\s*true").unwrap();
        new_code = and_true_pattern.replace_all(&new_code, "$1").to_string();
        
        // x && false -> false
        let and_false_pattern = Regex::new(r"(\w+)\s*&&\s*false").unwrap();
        new_code = and_false_pattern.replace_all(&new_code, "false").to_string();
        
        // x || true -> true
        let or_true_pattern = Regex::new(r"(\w+)\s*\|\|\s*true").unwrap();
        new_code = or_true_pattern.replace_all(&new_code, "true").to_string();
        
        // x || false -> x
        let or_false_pattern = Regex::new(r"(\w+)\s*\|\|\s*false").unwrap();
        new_code = or_false_pattern.replace_all(&new_code, "$1").to_string();
        
        ctx.code = new_code;
        Ok(())
    }

    /// 优化位掩码操作
    fn optimize_bit_masks(&mut self, ctx: &mut NhwcCtx) -> Result<()> {
        if self.debug {
            println!("BitsetPass: 优化位掩码操作");
        }
        
        let mut new_code = ctx.code.clone();
        
        // x & (1 << n) -> 检查第n位
        let bit_test_pattern = Regex::new(r"(\w+)\s*&\s*\(1\s*<<\s*(\d+)\)").unwrap();
        new_code = bit_test_pattern.replace_all(&new_code, "($1 >> $2) & 1").to_string();
        
        // x & ~(1 << n) -> 清除第n位
        let bit_clear_pattern = Regex::new(r"(\w+)\s*&\s*~\(1\s*<<\s*(\d+)\)").unwrap();
        new_code = bit_clear_pattern.replace_all(&new_code, "$1 & ~(1 << $2)").to_string();
        
        // x | (1 << n) -> 设置第n位
        let bit_set_pattern = Regex::new(r"(\w+)\s*\|\s*\(1\s*<<\s*(\d+)\)").unwrap();
        new_code = bit_set_pattern.replace_all(&new_code, "$1 | (1 << $2)").to_string();
        
        ctx.code = new_code;
        Ok(())
    }

    /// 优化位操作赋值
    fn optimize_bit_assignments(&mut self, ctx: &mut NhwcCtx) -> Result<()> {
        if self.debug {
            println!("BitsetPass: 优化位操作赋值");
        }
        
        let mut new_code = ctx.code.clone();
        
        // x <<= n -> 左移赋值
        let left_shift_assign_pattern = Regex::new(r"(\w+)\s*<<=\s*(\d+)").unwrap();
        new_code = left_shift_assign_pattern.replace_all(&new_code, |caps: &regex::Captures| {
            let var = &caps[1];
            let shift = caps[2].parse::<i32>().unwrap();
            if shift > 0 && shift <= 31 {
                let power = 1 << shift;
                let replacement = format!("{} *= {}", var, power);
                if self.debug {
                    println!("BitsetPass: 优化左移赋值 {} <<= {} -> {}", var, shift, replacement);
                }
                replacement
            } else {
                caps[0].to_string()
            }
        }).to_string();
        
        // x >>= n -> 右移赋值
        let right_shift_assign_pattern = Regex::new(r"(\w+)\s*>>=\s*(\d+)").unwrap();
        new_code = right_shift_assign_pattern.replace_all(&new_code, |caps: &regex::Captures| {
            let var = &caps[1];
            let shift = caps[2].parse::<i32>().unwrap();
            if shift > 0 && shift <= 31 {
                let power = 1 << shift;
                let replacement = format!("{} /= {}", var, power);
                if self.debug {
                    println!("BitsetPass: 优化右移赋值 {} >>= {} -> {}", var, shift, replacement);
                }
                replacement
            } else {
                caps[0].to_string()
            }
        }).to_string();
        
        ctx.code = new_code;
        Ok(())
    }

    /// 优化bool数组到字节的转换
    fn optimize_bool_array_to_byte(&mut self, ctx: &mut NhwcCtx) -> Result<()> {
        if self.debug {
            println!("BitsetPass: 优化bool数组到字节的转换");
        }
        
        let mut new_code = ctx.code.clone();
        
        // bool flags[8]; -> uint8_t flags;
        let bool_array_pattern = Regex::new(r"bool\s+(\w+)\s*\[\s*8\s*\]\s*;").unwrap();
        new_code = bool_array_pattern.replace_all(&new_code, "uint8_t $1;").to_string();
        
        // flags[0] = true; -> flags |= (1 << 0);
        let bool_array_assign_pattern = Regex::new(r"(\w+)\s*\[\s*(\d+)\s*\]\s*=\s*true\s*;").unwrap();
        new_code = bool_array_assign_pattern.replace_all(&new_code, "$1 |= (1 << $2);").to_string();
        
        // flags[0] = false; -> flags &= ~(1 << 0);
        let bool_array_assign_false_pattern = Regex::new(r"(\w+)\s*\[\s*(\d+)\s*\]\s*=\s*false\s*;").unwrap();
        new_code = bool_array_assign_false_pattern.replace_all(&new_code, "$1 &= ~(1 << $2);").to_string();
        
        ctx.code = new_code;
        Ok(())
    }
}

impl Pass for BitsetPass {
    fn run(&mut self, ctx: &mut NhwcCtx) -> Result<()> { 
        if self.debug {
            println!("BitsetPass: 开始执行位集优化");
        }
        
        self.optimize_shift_expressions(ctx)?;
        self.optimize_and_expressions(ctx)?;
        self.optimize_or_expressions(ctx)?;
        self.optimize_logical_expressions(ctx)?;
        self.optimize_bit_masks(ctx)?;
        self.optimize_bit_assignments(ctx)?;
        self.optimize_bool_array_to_byte(ctx)?;
        
        if self.debug {
            println!("BitsetPass: 位集优化完成");
        }
        
        Ok(())
    }

    fn get_desc(&self) -> String { 
        "位集优化：优化位操作、移位、逻辑表达式和bool数组转换".to_string()
    }

    fn get_pass_name(&self) -> String { 
        "BitsetPass".to_string()
    }
} 