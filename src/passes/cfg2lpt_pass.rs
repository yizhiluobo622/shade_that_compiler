use crate::toolkit::{context::NhwcCtx, dot::Config, etc::{dfs_with_priority, generate_png_by_graph_multi_tasks}, pass_manager::Pass, cfg_edge::CfgEdgeType, cfg_node::CfgNodeType, loop_node::{LoopTree, LoopNode}};
use crate::{add_node, add_node_with_edge, direct_parent_nodes, direct_child_nodes, node, node_mut};
use anyhow::*;
use ahash::{AHashMap, AHashSet};
use petgraph::visit::{EdgeRef, NodeRef};
use petgraph::graph::{DiGraph, NodeIndex, EdgeIndex};

/// 循环类型枚举
#[derive(Debug, Clone, PartialEq)]
enum LoopType {
    Reducible,    // 规约循环（单入口）
    Irreducible,  // 不可规约循环（多入口）
}

/// 循环信息结构
#[derive(Debug, Clone)]
struct LoopInfo {
    header: u32,           // 循环头节点（规约循环）或入口节点列表（不可规约循环）
    headers: Vec<u32>,     // 所有入口节点（不可规约循环）
    body: AHashSet<u32>,   // 循环体节点
    exits: Vec<u32>,       // 循环出口节点
    parent: Option<u32>,   // 父循环头
    children: Vec<u32>,    // 子循环头列表
    depth: usize,          // 嵌套深度
    back_edges: Vec<(u32, u32)>, // 回边 (tail, head)
    loop_type: LoopType,   // 循环类型
    transformed_graph: Option<DiGraph<u32, u32>>, // 变换后的图（不可规约循环）
    theta_node: Option<u32>, // θ节点（不可规约循环变换）
}

impl LoopInfo {
    fn new_reducible(header: u32) -> Self {
        Self {
            header,
            headers: vec![header],
            body: AHashSet::new(),
            exits: Vec::new(),
            parent: None,
            children: Vec::new(),
            depth: 0,
            back_edges: Vec::new(),
            loop_type: LoopType::Reducible,
            transformed_graph: None,
            theta_node: None,
        }
    }
    
    fn new_irreducible(headers: Vec<u32>) -> Self {
        Self {
            header: headers[0], // 使用第一个入口作为主循环头
            headers,
            body: AHashSet::new(),
            exits: Vec::new(),
            parent: None,
            children: Vec::new(),
            depth: 0,
            back_edges: Vec::new(),
            loop_type: LoopType::Irreducible,
            transformed_graph: None,
            theta_node: None,
        }
    }
    
    fn is_reducible(&self) -> bool {
        self.loop_type == LoopType::Reducible
    }
    
    fn is_irreducible(&self) -> bool {
        self.loop_type == LoopType::Irreducible
    }
}

pub struct Cfg2LptPass {
    pub is_gen_png: bool,
}

impl Cfg2LptPass {
    pub fn new(is_gen_png: bool) -> Self {
        Self { is_gen_png }
    }

    /// 基于支配树检查节点是否是循环头
    fn is_loop_header(&self, node: u32, ctx: &NhwcCtx) -> bool {
        // 检查是否有回边指向该节点
        for edge in ctx.cfg_graph.edges_directed(petgraph::graph::NodeIndex::new(node as usize), petgraph::Direction::Incoming) {
            let source = edge.source().index() as u32;
            
            // 检查是否是回边（基于支配关系）
            if self.is_back_edge(source, node, ctx) {
                return true;
            }
        }
        false
    }

    /// 基于支配树检查是否是回边
    fn is_back_edge(&self, tail: u32, head: u32, ctx: &NhwcCtx) -> bool {
        // 回边的定义：tail -> head，且head支配tail
        // 在支配树中，如果head是tail的祖先，则head支配tail
        
        // 获取tail和head对应的DJ节点
        let tail_dj = match ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(tail as usize)).unwrap().get_cor_dj_node() {
            std::result::Result::Ok(dj) => dj,
            Err(_) => return false,
        };
        
        let head_dj = match ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(head as usize)).unwrap().get_cor_dj_node() {
            std::result::Result::Ok(dj) => dj,
            Err(_) => return false,
        };
        
        // 检查head是否支配tail（在支配树中head是否是tail的祖先）
        self.is_dominator(*head_dj, *tail_dj, ctx)
    }

    /// 检查一个DJ节点是否支配另一个DJ节点
    fn is_dominator(&self, dominator: u32, dominated: u32, ctx: &NhwcCtx) -> bool {
        let mut current = dominated;
        
        // 沿着支配树向上遍历，直到找到dominator或到达根节点
        while current != dominator {
            // 获取当前节点的父节点（在支配树中）
            let parent_nodes: Vec<_> = ctx.dj_graph.edges_directed(petgraph::graph::NodeIndex::new(current as usize), petgraph::Direction::Incoming)
                .filter(|e| e.weight().is_dom())
                .map(|e| e.source().index() as u32)
                .collect();
            let parent = match parent_nodes.first() {
                Some(&parent) => parent,
                None => return false, // 到达根节点，说明dominator不是祖先
            };
            
            if parent == current {
                return false; // 防止循环
            }
            
            current = parent;
        }
        
        true
    }

    /// 基于支配树识别规约循环
    fn identify_reducible_loops(&self, ctx: &NhwcCtx) -> Result<Vec<LoopInfo>> {
        let mut loops = Vec::new();
        let mut visited_headers = AHashSet::new();
        
        // 遍历所有CFG节点，寻找循环头
        for node_idx in ctx.cfg_graph.node_indices() {
            let node = node_idx.index() as u32;
            
            if visited_headers.contains(&node) {
                continue;
            }
            
            if self.is_loop_header(node, ctx) {
                println!("发现规约循环头: {}", node);
                visited_headers.insert(node);
                
                // 识别循环信息
                let loop_info = self.identify_loop_from_header(node, ctx)?;
                loops.push(loop_info);
            }
        }
        
        Ok(loops)
    }

    /// 基于支配树识别不可规约循环
    fn identify_irreducible_loops(&self, ctx: &NhwcCtx) -> Result<Vec<LoopInfo>> {
        let mut irreducible_loops = Vec::new();
        let mut visited_nodes = AHashSet::new();
        
        // 使用强连通分量分析
        let sccs = petgraph::algo::tarjan_scc(&ctx.cfg_graph);
        
        for scc in sccs {
            if scc.len() <= 1 {
                continue; // 至少两个节点才可能是循环
            }
            
            let component: AHashSet<u32> = scc.iter().map(|&idx| idx.index() as u32).collect();
            
            // 检查是否已被规约循环包含
            let mut is_contained_by_reducible = false;
            for node in &component {
                if visited_nodes.contains(node) {
                    is_contained_by_reducible = true;
                    break;
                }
            }
            
            if is_contained_by_reducible {
                continue;
            }
            
            // 找到入口点
            let entry_points = self.find_entry_points(&component, ctx)?;
            
            // 检查是否是不可规约循环（多个入口点或支配关系异常）
            let mut is_irreducible = entry_points.len() > 1;
            
            if !is_irreducible && entry_points.len() == 1 {
                let header = entry_points[0];
                // 检查是否有节点不被循环头支配
                for &node in &component {
                    if node != header {
                        let header_dj = match ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(header as usize)).unwrap().get_cor_dj_node() {
                            std::result::Result::Ok(dj) => dj,
                            Err(_) => continue,
                        };
                        
                        let node_dj = match ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(node as usize)).unwrap().get_cor_dj_node() {
                            std::result::Result::Ok(dj) => dj,
                            Err(_) => continue,
                        };
                        
                        if !self.is_dominator(*header_dj, *node_dj, ctx) {
                            is_irreducible = true;
                            break;
                        }
                    }
                }
            }
            
            if is_irreducible {
                println!("发现不可规约循环，入口点: {:?}", entry_points);
                println!("  循环体: {:?}", component);
                
                // 创建不可规约循环信息
                let mut loop_info = LoopInfo::new_irreducible(entry_points.clone());
                loop_info.body = component.clone();
                
                // 识别回边
                for &node in &component {
                    for edge in ctx.cfg_graph.edges_directed(petgraph::graph::NodeIndex::new(node as usize), petgraph::Direction::Incoming) {
                        let source = edge.source().index() as u32;
                        if component.contains(&source) && entry_points.contains(&(edge.target().index() as u32)) {
                            loop_info.back_edges.push((source, edge.target().index() as u32));
                        }
                    }
                }
                
                // 识别循环出口
                for &node in &component {
                    for edge in ctx.cfg_graph.edges(petgraph::graph::NodeIndex::new(node as usize)) {
                        let target = edge.target().index() as u32;
                        if !component.contains(&target) {
                            loop_info.exits.push(target);
                        }
                    }
                }
                
                irreducible_loops.push(loop_info);
                
                // 标记已访问的节点
                for &node in &component {
                    visited_nodes.insert(node);
                }
            }
        }
        
        Ok(irreducible_loops)
    }

    /// 找到循环的入口点
    fn find_entry_points(&self, component: &AHashSet<u32>, ctx: &NhwcCtx) -> Result<Vec<u32>> {
        let mut entry_points = Vec::new();
        
        for &node in component {
            // 检查是否有来自循环外的边指向该节点
            for edge in ctx.cfg_graph.edges_directed(petgraph::graph::NodeIndex::new(node as usize), petgraph::Direction::Incoming) {
                let source = edge.source().index() as u32;
                if !component.contains(&source) {
                    entry_points.push(node);
                    break;
                }
            }
        }
        
        Ok(entry_points)
    }

    /// 从循环头识别循环（基于支配树）
    fn identify_loop_from_header(&self, header: u32, ctx: &NhwcCtx) -> Result<LoopInfo> {
        let mut loop_body = AHashSet::new();
        let mut back_edges = Vec::new();
        let mut exits = Vec::new();
        
        // 添加循环头到循环体
        loop_body.insert(header);
        
        // 识别回边并收集循环体
        for edge in ctx.cfg_graph.edges_directed(petgraph::graph::NodeIndex::new(header as usize), petgraph::Direction::Incoming) {
            let source = edge.source().index() as u32;
            if self.is_back_edge(source, header, ctx) {
                back_edges.push((source, header));
                loop_body.insert(source);
            }
        }
        
        // 使用工作列表算法扩展循环体
        let mut worklist: Vec<u32> = loop_body.iter().cloned().collect();
        
        while let Some(node) = worklist.pop() {
            // 处理前驱节点
            for edge in ctx.cfg_graph.edges_directed(petgraph::graph::NodeIndex::new(node as usize), petgraph::Direction::Incoming) {
                let pred = edge.source().index() as u32;
                
                // 检查前驱是否应该包含在循环中
                if !loop_body.contains(&pred) {
                    // 获取前驱和循环头的DJ节点
                    let pred_dj = match ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(pred as usize)).unwrap().get_cor_dj_node() {
                        std::result::Result::Ok(dj) => dj,
                        Err(_) => continue,
                    };
                    
                    let header_dj = match ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(header as usize)).unwrap().get_cor_dj_node() {
                        std::result::Result::Ok(dj) => dj,
                        Err(_) => continue,
                    };
                    
                    // 关键修改：更精确的支配关系检查
                    if self.is_dominator(*header_dj, *pred_dj, ctx) 
                        && self.is_reachable_from(header, pred, ctx)
                        && !self.is_in_other_loop(pred, header, ctx) 
                    {
                        loop_body.insert(pred);
                        worklist.push(pred);
                    }
                }
            }
        }
        
        // 识别循环出口
        for &node in &loop_body {
            for edge in ctx.cfg_graph.edges(petgraph::graph::NodeIndex::new(node as usize)) {
                let target = edge.target().index() as u32;
                if !loop_body.contains(&target) {
                    exits.push(target);
                }
            }
        }
        
        Ok(LoopInfo {
            header,
            headers: vec![header],
            body: loop_body,
            exits,
            parent: None,
            children: Vec::new(),
            depth: 0,
            back_edges,
            loop_type: LoopType::Reducible,
            transformed_graph: None,
            theta_node: None,
        })
    }

    /// 检查从start节点是否可达target节点
    fn is_reachable_from(&self, start: u32, target: u32, ctx: &NhwcCtx) -> bool {
        let mut visited = AHashSet::new();
        let mut stack = vec![start];
        
        while let Some(current) = stack.pop() {
            if current == target {
                return true;
            }
            
            if visited.contains(&current) {
                continue;
            }
            
            visited.insert(current);
            
            // 遍历所有后继节点
            for edge in ctx.cfg_graph.edges(petgraph::graph::NodeIndex::new(current as usize)) {
                let successor = edge.target().index() as u32;
                if !visited.contains(&successor) {
                    stack.push(successor);
                }
            }
        }
        
        false
    }

    /// 检查节点是否属于其他循环
    fn is_in_other_loop(&self, node: u32, exclude_header: u32, ctx: &NhwcCtx) -> bool {
        // 简单实现：检查节点是否被其他循环头支配
        // 这里可以根据需要扩展更复杂的逻辑
        for node_idx in ctx.cfg_graph.node_indices() {
            let potential_header = node_idx.index() as u32;
            if potential_header == exclude_header {
                continue;
            }
            
            // 检查是否是循环头
            if self.is_loop_header(potential_header, ctx) {
                let header_dj = match ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(potential_header as usize)).unwrap().get_cor_dj_node() {
                    std::result::Result::Ok(dj) => dj,
                    Err(_) => continue,
                };
                
                let node_dj = match ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(node as usize)).unwrap().get_cor_dj_node() {
                    std::result::Result::Ok(dj) => dj,
                    Err(_) => continue,
                };
                
                if self.is_dominator(*header_dj, *node_dj, ctx) {
                    return true;
                }
            }
        }
        
        false
    }

    /// 基于支配树构建循环嵌套关系
    fn build_nesting_relationships(&self, loops: &mut AHashMap<u32, LoopInfo>, ctx: &NhwcCtx) -> Result<()> {
        // 按循环头支配关系排序
        let mut loop_headers: Vec<u32> = loops.keys().cloned().collect();
        loop_headers.sort_by(|a, b| {
            let a_dj = match ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(*a as usize)).unwrap().get_cor_dj_node() {
                std::result::Result::Ok(dj) => *dj,
                Err(_) => 0,
            };
            let b_dj = match ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(*b as usize)).unwrap().get_cor_dj_node() {
                std::result::Result::Ok(dj) => *dj,
                Err(_) => 0,
            };
            a_dj.cmp(&b_dj)
        });
        
        // 构建嵌套关系
        for i in 0..loop_headers.len() {
            let header = loop_headers[i];
            let loop_info = loops[&header].clone();
            
            // 寻找直接父循环
            for j in (0..i).rev() {
                let potential_parent = loop_headers[j];
                let parent_loop = &loops[&potential_parent];
                
                // 检查是否应该嵌套
                if self.should_nest(&loop_info, parent_loop, ctx) {
                    loops.get_mut(&header).unwrap().parent = Some(potential_parent);
                    loops.get_mut(&potential_parent).unwrap().children.push(header);
                    println!("发现嵌套关系：循环 {} 嵌套在循环 {} 内部", header, potential_parent);
                    break;
                }
            }
        }
        
        // 计算嵌套深度
        for &header in &loop_headers {
            self.compute_loop_depth(header, loops, 0);
        }
        
        Ok(())
    }

    /// 检查循环嵌套关系
    fn should_nest(&self, child: &LoopInfo, parent: &LoopInfo, ctx: &NhwcCtx) -> bool {
        // 1. 检查子循环头是否在父循环体内
        if !parent.body.contains(&child.header) {
            return false;
        }
        
        // 2. 检查支配关系
        let child_dj = match ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(child.header as usize)).unwrap().get_cor_dj_node() {
            std::result::Result::Ok(dj) => dj,
            Err(_) => return false,
        };
        
        let parent_dj = match ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(parent.header as usize)).unwrap().get_cor_dj_node() {
            std::result::Result::Ok(dj) => dj,
            Err(_) => return false,
        };
        
        self.is_dominator(*parent_dj, *child_dj, ctx)
    }

    /// 计算循环嵌套深度
    fn compute_loop_depth(&self, header: u32, loops: &mut AHashMap<u32, LoopInfo>, depth: usize) {
        let mut visited = AHashSet::new();
        self.compute_loop_depth_inner(header, loops, depth, &mut visited);
    }
    
    fn compute_loop_depth_inner(&self, header: u32, loops: &mut AHashMap<u32, LoopInfo>, depth: usize, visited: &mut AHashSet<u32>) {
        if visited.contains(&header) {
            return; // 防止循环引用
        }
        
        if depth > 1000 {
            println!("警告：循环嵌套深度超过1000，可能存在循环引用");
            return; // 防止栈溢出
        }
        
        visited.insert(header);
        
        if let Some(loop_info) = loops.get_mut(&header) {
            loop_info.depth = depth;
            
            // 递归计算子循环深度
            for &child_header in &loop_info.children.clone() {
                self.compute_loop_depth_inner(child_header, loops, depth + 1, visited);
            }
        }
    }

    /// 构建循环嵌套树 - 基于支配树的遍历
    fn construct_loop_tree(&self, ctx: &NhwcCtx, loops: &AHashMap<u32, LoopInfo>) -> Result<LoopTree> {
        let mut tree = LoopTree::new();
        
        // 添加根节点
        let root_idx = tree.add_node(LoopNode::new_root_node());
        
        // 遍历所有函数的入口节点
        for (func_symidx, cfg_entry) in ctx.symtab.get_global_info().get_all_cfg_func_symidx_entry_tuples()?.clone() {
            println!("处理函数: {:?}, 入口节点: {}", func_symidx, cfg_entry);
            
            // 添加函数节点
            let func_node = LoopNode::new_func_node(cfg_entry, ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(cfg_entry as usize)).unwrap().cfg_node_type.clone());
            let func_idx = tree.add_node(func_node);
            tree.add_edge(root_idx, func_idx, ());
            println!("创建函数节点: {} -> 树节点: {:?}", cfg_entry, func_idx);
            
            // 将函数入口节点映射到函数节点
            let mut node_to_tree_node = AHashMap::new();
            node_to_tree_node.insert(cfg_entry, func_idx);
            
            // 使用支配树遍历CFG
            let dfs_nodes = dfs_with_priority(&ctx.cfg_graph, cfg_entry, |e| match &e.weight().cfg_edge_type {
                CfgEdgeType::BodyHead { } => 1,
                CfgEdgeType::IfFalse { } => 2,
                CfgEdgeType::Direct { } => 2,
                CfgEdgeType::IfTrue { } => 1,
                CfgEdgeType::BodyTail { } => -1,
            });
            
            println!("DFS遍历节点: {:?}", dfs_nodes);
            let mut visited_nodes = AHashSet::new();
            
            for node in dfs_nodes {
                let node_idx = node;
                
                // 避免重复处理节点
                if visited_nodes.contains(&node_idx) {
                    continue;
                }
                visited_nodes.insert(node_idx);
                
                let node_type = ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(node_idx as usize)).unwrap().cfg_node_type.clone();
                
                // 检查是否是循环头
                if let Some(loop_info) = loops.get(&node_idx) {
                    println!("发现循环头: {} (类型: {:?})", node_idx, node_type);
                    
                    // 创建循环节点（循环头）
                    let loop_idx = tree.add_node(LoopNode::new_loop_node(node_idx, node_type));
                    println!("创建循环节点: {} -> 树节点: {:?}", node_idx, loop_idx);
                    
                    // 检查当前循环应该挂在哪个父节点下
                    let mut parent_node = func_idx;
                    
                    // 使用支配树关系确定父节点
                    if let Some(parent_header) = loop_info.parent {
                        // 找到父循环的循环节点
                        if let Some(parent_loop_idx) = node_to_tree_node.get(&parent_header) {
                            parent_node = *parent_loop_idx;
                            let loop_type_str = if loop_info.is_reducible() { "规约" } else { "不可规约" };
                            println!("{}循环头 {} 嵌套在循环 {} 内部", loop_type_str, node_idx, parent_header);
                        }
                    }
                    
                    let loop_type_str = if loop_info.is_reducible() { "规约" } else { "不可规约" };
                    println!("{}循环头 {} 的循环节点挂在父节点 {:?} 下", loop_type_str, node_idx, parent_node);
                    tree.add_edge(parent_node, loop_idx, ());
                    node_to_tree_node.insert(node_idx, loop_idx);
                    
                    // 添加循环体中的所有节点作为循环内部节点
                    println!("循环体节点: {:?}", loop_info.body);
                    for &body_node in &loop_info.body {
                        if body_node != node_idx { // 避免重复添加循环头
                            // 检查节点是否已经被处理过
                            if !node_to_tree_node.contains_key(&body_node) {
                                // 特殊处理：函数入口节点已经在上面创建了，跳过
                                if body_node == cfg_entry {
                                    println!("跳过循环体中的函数入口节点: {}", body_node);
                                    continue;
                                }
                                
                                let body_node_type = ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(body_node as usize)).unwrap().cfg_node_type.clone();
                                let body_tree_node = tree.add_node(LoopNode::new_terminal_node(body_node, body_node_type));
                                tree.add_edge(loop_idx, body_tree_node, ());
                                node_to_tree_node.insert(body_node, body_tree_node);
                                println!("添加循环体节点: {} -> 树节点: {:?}", body_node, body_tree_node);
                            } else {
                                println!("循环体节点 {} 已经被处理过", body_node);
                            }
                        }
                    }
                    
                    // 对于不可规约循环，特殊处理多入口关系
                    if loop_info.is_irreducible() {
                        // 添加θ节点
                        if let Some(theta_node) = loop_info.theta_node {
                            let theta_tree_node = tree.add_node(LoopNode::new_terminal_node(theta_node, CfgNodeType::BasicBlock { ast_nodes: vec![] }));
                            tree.add_edge(loop_idx, theta_tree_node, ());
                            node_to_tree_node.insert(theta_node, theta_tree_node);
                            println!("为不可规约循环 {} 添加θ节点 {}", node_idx, theta_node);
                        }
                        
                        // 为每个入口创建虚拟分支节点，可视化多入口关系
                        for &entry in &loop_info.headers {
                            if entry != node_idx && !node_to_tree_node.contains_key(&entry) { // 避免重复添加
                                // 特殊处理：函数入口节点已经在上面创建了，跳过
                                if entry == cfg_entry {
                                    println!("跳过不可规约循环中的函数入口节点: {}", entry);
                                    continue;
                                }
                                
                                let entry_tree_node = tree.add_node(LoopNode::new_terminal_node(entry, CfgNodeType::BasicBlock { ast_nodes: vec![] }));
                                tree.add_edge(loop_idx, entry_tree_node, ());
                                node_to_tree_node.insert(entry, entry_tree_node);
                                println!("为不可规约循环 {} 添加入口节点 {}", node_idx, entry);
                            }
                        }
                    }
                } else {
                    // 普通节点（不在任何循环内）
                    // 检查节点是否已经被处理过（避免重复创建Entry节点）
                    if !node_to_tree_node.contains_key(&node_idx) {
                        // 特殊处理：函数入口节点已经在上面创建了，跳过
                        if node_idx == cfg_entry {
                            println!("跳过重复的函数入口节点: {}", node_idx);
                            continue;
                        }
                        
                        let tree_node_idx = tree.add_node(LoopNode::new_terminal_node(node_idx, node_type.clone()));
                        node_to_tree_node.insert(node_idx, tree_node_idx);
                        println!("添加普通节点: {} (类型: {:?})", node_idx, node_type);
                        
                        // 找到包含当前节点的最内层循环（基于支配树）
                        let mut parent_node = func_idx;
                        let mut innermost_loop = None;
                        let mut max_depth = 0;
                        
                        // 检查节点属于哪个最内层循环
                        for (header, loop_info) in loops {
                            if loop_info.body.contains(&node_idx) {
                                if loop_info.depth > max_depth {
                                    max_depth = loop_info.depth;
                                    innermost_loop = Some(header);
                                }
                            }
                        }
                        
                        // 如果节点属于某个循环，挂在最内层循环下
                        if let Some(loop_header) = innermost_loop {
                            if let Some(&loop_tree_idx) = node_to_tree_node.get(&loop_header) {
                                parent_node = loop_tree_idx;
                                println!("节点 {} 属于循环 {} (深度: {})", node_idx, loop_header, max_depth);
                            }
                        }
                        
                        // 特殊处理：Gather节点（函数结束）应该挂在函数级别
                        if matches!(node_type, CfgNodeType::Gather { }) {
                            parent_node = func_idx;
                        }
                        
                        tree.add_edge(parent_node, tree_node_idx, ());
                    }
                }
            }
        }
        
        // 打印构建的循环嵌套树结构
        println!("=== 循环嵌套树构建完成 ===");
        println!("总节点数: {}", tree.node_count());
        println!("总边数: {}", tree.edge_count());
        
        Ok(tree)
    }
}

impl Pass for Cfg2LptPass {
    fn run(&mut self, ctx: &mut NhwcCtx) -> Result<()> {
        println!("开始基于支配树构建循环嵌套树...");
        
        // 确保支配树已经构建
        if ctx.dj_graph.node_count() == 0 {
            return Err(anyhow!("支配树尚未构建，请先运行Ncfg2DjgPass"));
        }
        
        // 1. 识别规约循环（基于支配树）
        println!("开始识别规约循环...");
        let reducible_loops = self.identify_reducible_loops(ctx)?;
        
        // 2. 识别不可规约循环
        println!("开始识别不可规约循环...");
        let irreducible_loops = self.identify_irreducible_loops(ctx)?;
        
        // 3. 合并所有循环
        let mut all_loops = AHashMap::new();
        
        for loop_info in reducible_loops {
            let header = loop_info.header;
            println!("发现规约循环头: {}, 循环体大小: {}", header, loop_info.body.len());
            all_loops.insert(header, loop_info);
        }
        
        for loop_info in irreducible_loops {
            let header = loop_info.header;
            println!("发现不可规约循环头: {}, 入口点: {:?}", header, loop_info.headers);
            all_loops.insert(header, loop_info);
        }
        
        // 4. 构建循环嵌套关系（基于支配树）
        println!("开始构建循环嵌套关系...");
        self.build_nesting_relationships(&mut all_loops, ctx)?;
        
        // 打印嵌套关系
        for (header, loop_info) in &all_loops {
            let loop_type_str = if loop_info.is_reducible() { "规约" } else { "不可规约" };
            println!("{}循环头 {}: 父循环={:?}, 子循环={:?}", 
                loop_type_str, header, loop_info.parent, loop_info.children);
        }
        
        // 5. 构建循环嵌套树
        let loop_tree = self.construct_loop_tree(ctx, &all_loops)?;
        
        // 6. 存储到上下文中
        ctx.loop_tree = loop_tree;
        
        // 7. 生成可视化图（如果需要）
        if self.is_gen_png {
            self.generate_debug_graph(ctx, &ctx.loop_tree)?;
        }
        
        println!("基于支配树的循环嵌套树构建完成，识别到 {} 个循环", all_loops.len());
        
        // 打印循环信息
        for (header, loop_info) in &all_loops {
            let loop_type_str = if loop_info.is_reducible() { "规约" } else { "不可规约" };
            println!("{}循环头: {}, 深度: {}, 循环体大小: {}, 子循环: {:?}", 
                loop_type_str, header, loop_info.depth, loop_info.body.len(), loop_info.children);
            
            if loop_info.is_irreducible() {
                println!("  - 入口点: {:?}", loop_info.headers);
                if let Some(theta_node) = loop_info.theta_node {
                    println!("  - θ节点: {}", theta_node);
                }
            }
        }
        
        // 打印循环嵌套树结构
        println!("循环嵌套树节点数: {}", ctx.loop_tree.node_count());
        println!("循环嵌套树边数: {}", ctx.loop_tree.edge_count());
        
        // 验证循环嵌套树是否真的被构建
        if ctx.loop_tree.node_count() > 0 {
            println!("✅ 基于支配树的循环嵌套树构建成功！");
            println!("   - 根节点: {}", ctx.loop_tree.node_count() > 0);
            println!("   - 树结构: {} 个节点, {} 条边", ctx.loop_tree.node_count(), ctx.loop_tree.edge_count());
        } else {
            println!("❌ 循环嵌套树构建失败！");
        }
        
        Ok(())
    }

    fn get_desc(&self) -> String {
        "基于支配树构建循环嵌套树".to_string()
    }

    fn get_pass_name(&self) -> String {
        "Cfg2LptPass".to_string()
    }
}

impl Cfg2LptPass {
    /// 生成调试图
    fn generate_debug_graph(&self, ctx: &NhwcCtx, loop_tree: &LoopTree) -> Result<()> {
        println!("生成循环嵌套树调试图");
        
        // 使用现有的图片生成功能
        generate_png_by_graph_multi_tasks(
            loop_tree,
            "loop_nesting_tree".to_string(),
            &[
                Config::EdgeNoLabel,
                Config::Title("Loop Nesting Tree (Based on Dominator Tree)".to_string()),
                Config::RankDirLR,
            ],
            &mut Vec::new()
        )?;
        
        println!("循环嵌套树图片已生成: loop_nesting_tree.png");
        Ok(())
    }
}
