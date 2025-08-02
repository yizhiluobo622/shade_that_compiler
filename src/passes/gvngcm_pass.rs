use crate::{make_field_trait_for_struct, node, node_mut, reg_field_for_struct, toolkit::{cfg_node::{CfgNode, CFG_ROOT}, context::NhwcCtx, dot::Config, etc::{dfs, dfs_with_priority, generate_png_by_graph_multi_tasks}, eval_et, gcm::gcm, gen_instr_et::parse_instr_list_to_et, gvn::gvn, pass_manager::Pass, symtab::WithBorrow}};
use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use anyhow::*;
use crate::toolkit::field::Field;
/// 定义额外的信息，这样我们就可以把 add_field 宏加入到符号表或者任何实现了 Fields trait 的地方
/// 任何一个Pass 都有一个pass_run函数 来进行这个pass 相关的工作，比如说对于 SSAPass 我们要对 一个BasicBlock 中的ExprTree做出转换。
/// 因为实际上 一个 ExprTree 最终会对应一个BasicBlock。
/// 可能会有人问，那我们为什么还要生成 nhwc_ir ？ 因为 nhwc_ir 有如下优点
/// 1. 便于debug，到时候这个pass 写完生成这个 cfg 对应的 llvm_ir 我就能更清楚的知道我们到底做了哪些更改
/// 2. nhwc_ir 是线性的结构，而 汇编语言也是线性的结构 ，这样就可以 从 nhwc_ir 转换成 汇编代码了
///
///
///
/// 总上，pass 的主要操作对象是 每个basic block 的expr_tree以及 cfg node。这点我们大概不会变
/// 这个结构体，用于存储与Pass 相关的数据
///
#[derive(Debug)]
pub struct GvnGcmPass {is_gen_instr_et:bool, is_gen_gvngcm_cfg:bool}
impl GvnGcmPass {
    pub fn new(is_gen_instr_et:bool, is_gen_gvngcm_cfg:bool) -> Self { GvnGcmPass { is_gen_instr_et, is_gen_gvngcm_cfg } }
}


impl Pass for GvnGcmPass {
    // 运行这个pass
    fn run(&mut self, ctx:&mut NhwcCtx) -> Result<()> { 
        //println!("GvnGcmPass: 开始GVN/GCM优化");
        
        // 在GVN/GCM优化前记录数组相关的变量
        let mut array_vars_before = Vec::new();
        for (_, cfg_entry) in ctx.symtab.get_global_info().get_all_cfg_func_symidx_entry_tuples()?.clone() {
            let dfs_node_vec = dfs_with_priority(&ctx.cfg_graph, cfg_entry, |e| match &e.weight().cfg_edge_type {
                crate::toolkit::cfg_edge::CfgEdgeType::BodyHead { } => 1,
                crate::toolkit::cfg_edge::CfgEdgeType::IfFalse { } => 2,
                crate::toolkit::cfg_edge::CfgEdgeType::Direct { } => 2,
                crate::toolkit::cfg_edge::CfgEdgeType::IfTrue { } => 1,
                crate::toolkit::cfg_edge::CfgEdgeType::BodyTail { } => 1,
            });

            for &cfg_node in dfs_node_vec.iter() {
                let cfg_graph = &ctx.cfg_graph;
                let cfg_node_struct = node!(at cfg_node in cfg_graph);
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
                                //println!("  GVN/GCM前发现数组相关变量: {} -> {}", ptr_name, instr_struct.text);
                            }
                        },
                        crate::toolkit::nhwc_instr::NhwcInstrType::GetElementPtr { ptr_symidx, .. } => {
                            let ptr_name = ptr_symidx.as_ref_borrow().symbol_name.clone();
                            if self.is_array_related(&ptr_name) {
                                array_vars_before.push((ptr_name.clone(), instr_struct.text.clone()));
                                //println!("  GVN/GCM前发现数组相关变量: {} -> {}", ptr_name, instr_struct.text);
                            }
                        },
                        _ => {}
                    }
                }
            }
        }
        
        //println!("  GVN/GCM前发现 {} 个数组相关变量", array_vars_before.len());
        
        // for gvn we should traverse through the cfg node of dominant tree 

        let dom_tree= &mut ctx.dj_graph;
        let cfg_graph= &mut ctx.cfg_graph;
        let symtab= &mut ctx.symtab;
        let instr_slab= &mut ctx.nhwc_instr_slab;
        let scope_tree= &mut ctx.scope_tree;
        let instr_et = &mut ctx.instr_et;
        let &dj_root = node!(at CFG_ROOT in cfg_graph).get_cor_dj_node()?;

        let rst=gcm(instr_et,  cfg_graph, symtab, instr_slab, scope_tree, &dom_tree)
            .and(gvn(instr_et,dom_tree, cfg_graph, symtab, instr_slab, scope_tree)).and(gcm(instr_et,  cfg_graph, symtab, instr_slab, scope_tree, &dom_tree));

        // let rst=gcm(instr_et,  cfg_graph, symtab, instr_slab, scope_tree, &dom_tree);
        // let rst = gvn(instr_et,dom_tree, cfg_graph, symtab, instr_slab, scope_tree);
        
        // 在GVN/GCM优化后记录数组相关的变量
        let mut array_vars_after = Vec::new();
        for (_, cfg_entry) in ctx.symtab.get_global_info().get_all_cfg_func_symidx_entry_tuples()?.clone() {
            let dfs_node_vec = dfs_with_priority(&ctx.cfg_graph, cfg_entry, |e| match &e.weight().cfg_edge_type {
                crate::toolkit::cfg_edge::CfgEdgeType::BodyHead { } => 1,
                crate::toolkit::cfg_edge::CfgEdgeType::IfFalse { } => 2,
                crate::toolkit::cfg_edge::CfgEdgeType::Direct { } => 2,
                crate::toolkit::cfg_edge::CfgEdgeType::IfTrue { } => 1,
                crate::toolkit::cfg_edge::CfgEdgeType::BodyTail { } => 1,
            });

            for &cfg_node in dfs_node_vec.iter() {
                let cfg_graph = &ctx.cfg_graph;
                let cfg_node_struct = node!(at cfg_node in cfg_graph);
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
                                //println!("  GVN/GCM后发现数组相关变量: {} -> {}", ptr_name, instr_struct.text);
                            }
                        },
                        crate::toolkit::nhwc_instr::NhwcInstrType::GetElementPtr { ptr_symidx, .. } => {
                            let ptr_name = ptr_symidx.as_ref_borrow().symbol_name.clone();
                            if self.is_array_related(&ptr_name) {
                                array_vars_after.push((ptr_name.clone(), instr_struct.text.clone()));
                                //println!("  GVN/GCM后发现数组相关变量: {} -> {}", ptr_name, instr_struct.text);
                            }
                        },
                        _ => {}
                    }
                }
            }
        }
        
        //println!("  GVN/GCM后发现 {} 个数组相关变量", array_vars_after.len());
        //println!("  GVN/GCM优化完成，变量数量变化: {} -> {}", array_vars_before.len(), array_vars_after.len());
        
        if self.is_gen_gvngcm_cfg{
            for (idx,instr_struct) in ctx.nhwc_instr_slab.iter_mut(){
                instr_struct.text.clear();
                instr_struct.load_idx_text(idx);
            }
            for cfg_node_struct in ctx.cfg_graph.node_weights_mut() {
                cfg_node_struct.clear_text();
                cfg_node_struct.load_ast_node_text(&ctx.ast_tree)?;
                cfg_node_struct.load_instrs_text(&ctx.nhwc_instr_slab)?;
            }
            generate_png_by_graph_multi_tasks(&ctx.cfg_graph.clone(), "gvngcm_cfg".to_string(), &[Config::Record, Config::Rounded, Config::Title("gvngcm_cfg".to_string()), Config::NodeIndexLabel, Config::CfgBlock],&mut ctx.io_task_list)?
        }
        if self.is_gen_instr_et {
            generate_png_by_graph_multi_tasks(&ctx.instr_et.clone(), "instr_et".to_string(), &[Config::Record, Config::Title("instr_et_tree".to_string()),Config::NodeIndexLabel],&mut ctx.io_task_list)?;
        }
        rst 
    }
    // 返回pass的描述，具体作用
    fn get_desc(&self) -> String { return "pass gvngcm description".to_string(); }

    // 返回pass的名称
    fn get_pass_name(&self) -> String { return "GvnGcmPass".to_string(); }
}

impl GvnGcmPass {
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
