use anyhow::{Result};

use crate::toolkit::{ dot::Config, etc::generate_png_by_graph_multi_tasks, gen_ssa::{add_phi_nodes, variable_renaming}, pass_manager::Pass, symtab::WithBorrow};
#[derive(Debug)]
pub struct SsaPass {is_gen_ssa_cfg_png:bool,is_gen_symtab_graph_png:bool}
impl SsaPass {
    pub fn new(is_gen_ssa_cfg_png:bool,is_gen_symtab_graph_png:bool) -> Self { SsaPass {is_gen_ssa_cfg_png,is_gen_symtab_graph_png} }
}

impl Pass for SsaPass {
   // 运行这个pass
    fn run(&mut self, ctx:&mut crate::toolkit::context::NhwcCtx) -> Result<()> { 
        println!("SSAPass: 开始SSA转换");
        
        // 在SSA转换前记录数组相关的变量
        let mut array_vars_before = Vec::new();
        for (_, cfg_entry) in ctx.symtab.get_global_info().get_all_cfg_func_symidx_entry_tuples()?.clone() {
            let dfs_node_vec = crate::toolkit::etc::dfs_with_priority(&ctx.cfg_graph, cfg_entry, |e| match &e.weight().cfg_edge_type {
                crate::toolkit::cfg_edge::CfgEdgeType::BodyHead { } => 1,
                crate::toolkit::cfg_edge::CfgEdgeType::IfFalse { } => 2,
                crate::toolkit::cfg_edge::CfgEdgeType::Direct { } => 2,
                crate::toolkit::cfg_edge::CfgEdgeType::IfTrue { } => 1,
                crate::toolkit::cfg_edge::CfgEdgeType::BodyTail { } => 1,
            });

            for &cfg_node in dfs_node_vec.iter() {
                let cfg_graph = &ctx.cfg_graph;
                let cfg_node_struct = crate::node!(at cfg_node in cfg_graph);
                let nhwc_instr_slab = &ctx.nhwc_instr_slab;

                for &instr_idx in &cfg_node_struct.instrs.instr_vec {
                    let instr_struct = crate::instr!(at instr_idx in nhwc_instr_slab)?;
                    
                    // 检查是否涉及数组访问
                    match &instr_struct.instr_type {
                        crate::toolkit::nhwc_instr::NhwcInstrType::Load { ptr_symidx, .. } | 
                        crate::toolkit::nhwc_instr::NhwcInstrType::Store { ptr_symidx, .. } => {
                            let ptr_name = ptr_symidx.as_ref_borrow().symbol_name.clone();
                            if self.is_array_related(&ptr_name) {
                                array_vars_before.push((ptr_name.clone(), instr_struct.text.clone()));
                                println!("  SSA前发现数组相关变量: {} -> {}", ptr_name, instr_struct.text);
                            }
                        },
                        crate::toolkit::nhwc_instr::NhwcInstrType::GetElementPtr { ptr_symidx, .. } => {
                            let ptr_name = ptr_symidx.as_ref_borrow().symbol_name.clone();
                            if self.is_array_related(&ptr_name) {
                                array_vars_before.push((ptr_name.clone(), instr_struct.text.clone()));
                                println!("  SSA前发现数组相关变量: {} -> {}", ptr_name, instr_struct.text);
                            }
                        },
                        _ => {}
                    }
                }
            }
        }
        
        println!("  SSA前发现 {} 个数组相关变量", array_vars_before.len());
        
        let add_phi_rst = add_phi_nodes(&mut ctx.cfg_graph, &mut ctx.dj_graph, &mut ctx.symtab, &mut ctx.nhwc_instr_slab);
        let variable_renaming_rst = variable_renaming(&mut ctx.cfg_graph, &mut ctx.dj_graph, &mut ctx.symtab, &mut ctx.nhwc_instr_slab);
        
        // 在SSA转换后记录数组相关的变量
        let mut array_vars_after = Vec::new();
        for (_, cfg_entry) in ctx.symtab.get_global_info().get_all_cfg_func_symidx_entry_tuples()?.clone() {
            let dfs_node_vec = crate::toolkit::etc::dfs_with_priority(&ctx.cfg_graph, cfg_entry, |e| match &e.weight().cfg_edge_type {
                crate::toolkit::cfg_edge::CfgEdgeType::BodyHead { } => 1,
                crate::toolkit::cfg_edge::CfgEdgeType::IfFalse { } => 2,
                crate::toolkit::cfg_edge::CfgEdgeType::Direct { } => 2,
                crate::toolkit::cfg_edge::CfgEdgeType::IfTrue { } => 1,
                crate::toolkit::cfg_edge::CfgEdgeType::BodyTail { } => 1,
            });

            for &cfg_node in dfs_node_vec.iter() {
                let cfg_graph = &ctx.cfg_graph;
                let cfg_node_struct = crate::node!(at cfg_node in cfg_graph);
                let nhwc_instr_slab = &ctx.nhwc_instr_slab;

                for &instr_idx in &cfg_node_struct.instrs.instr_vec {
                    let instr_struct = crate::instr!(at instr_idx in nhwc_instr_slab)?;
                    
                    // 检查是否涉及数组访问
                    match &instr_struct.instr_type {
                        crate::toolkit::nhwc_instr::NhwcInstrType::Load { ptr_symidx, .. } | 
                        crate::toolkit::nhwc_instr::NhwcInstrType::Store { ptr_symidx, .. } => {
                            let ptr_name = ptr_symidx.as_ref_borrow().symbol_name.clone();
                            if self.is_array_related(&ptr_name) {
                                array_vars_after.push((ptr_name.clone(), instr_struct.text.clone()));
                                println!("  SSA后发现数组相关变量: {} -> {}", ptr_name, instr_struct.text);
                            }
                        },
                        crate::toolkit::nhwc_instr::NhwcInstrType::GetElementPtr { ptr_symidx, .. } => {
                            let ptr_name = ptr_symidx.as_ref_borrow().symbol_name.clone();
                            if self.is_array_related(&ptr_name) {
                                array_vars_after.push((ptr_name.clone(), instr_struct.text.clone()));
                                println!("  SSA后发现数组相关变量: {} -> {}", ptr_name, instr_struct.text);
                            }
                        },
                        _ => {}
                    }
                }
            }
        }
        
        println!("  SSA后发现 {} 个数组相关变量", array_vars_after.len());
        println!("  SSA转换完成，变量数量变化: {} -> {}", array_vars_before.len(), array_vars_after.len());
        
        if self.is_gen_ssa_cfg_png{
            for (idx,instr_struct) in ctx.nhwc_instr_slab.iter_mut(){
                instr_struct.load_idx_text(idx);
            }
            for cfg_node_struct in ctx.cfg_graph.node_weights_mut() {
                cfg_node_struct.clear_text();
                cfg_node_struct.load_ast_node_text(&ctx.ast_tree)?;
                cfg_node_struct.load_instrs_text(&ctx.nhwc_instr_slab)?;
            }
            crate::toolkit::etc::generate_png_by_graph_multi_tasks(&ctx.cfg_graph.clone(), "ssa_cfg".to_string(), &[Config::Record, Config::Rounded, Config::Title("ssa_cfg".to_string()), Config::NodeIndexLabel, Config::CfgBlock],&mut ctx.io_task_list)?
        }
        if self.is_gen_symtab_graph_png {
            ctx.symtab_graph.clear();
            ctx.symtab.debug_symtab_graph(format!("after ssa pass"), &mut ctx.symtab_graph,vec![]);
            crate::toolkit::etc::generate_png_by_graph_multi_tasks(&ctx.symtab_graph.clone(), "ssa_symtab".to_string(), &[Config::Record, Config::Rounded, Config::Title("ssa_symtab".to_string()), Config::NodeIndexLabel, Config::CfgBlock],&mut ctx.io_task_list)?
        }
        add_phi_rst.and(variable_renaming_rst)

    }
    // 返回pass的描述，具体作用
    fn get_desc(&self) -> String { return "pass ssa description".to_string(); }

    // 返回pass的名称
    fn get_pass_name(&self) -> String { return "SSAPass".to_string(); }
}

impl SsaPass {
    // 检查是否为数组相关变量
    fn is_array_related(&self, var_name: &str) -> bool {
        var_name.contains("array") || var_name.contains("arr") || 
        var_name.contains("matrix") || var_name.contains("mat") ||
        var_name.contains("table") || var_name.contains("grid") ||
        (var_name.contains("_") && (var_name.contains("i") || var_name.contains("j"))) ||
        var_name == "a" || var_name == "b" || var_name == "c" ||
        var_name.contains("i") || var_name.contains("j") || var_name.contains("k") ||
        // 更广泛的检测：包含数字的变量名可能是数组访问
        (var_name.contains("_") && var_name.chars().any(|c| c.is_digit(10))) ||
        // 检测可能的数组指针
        var_name.contains("ptr") || var_name.contains("addr") ||
        // 检测可能的数组元素访问
        var_name.contains("elem") || var_name.contains("val")
    }
}

