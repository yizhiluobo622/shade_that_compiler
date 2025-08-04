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
        println!("  检查节点 {} 的入边...", node);
        
        // 首先检查节点类型
        let node_type = &ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(node as usize)).unwrap().cfg_node_type;
        if let CfgNodeType::WhileLoop { .. } = node_type {
            println!("    ✅ 节点 {} 是WhileLoop类型，直接识别为循环头", node);
            return true;
        }
        
        // 检查是否有回边指向该节点
        for edge in ctx.cfg_graph.edges_directed(petgraph::graph::NodeIndex::new(node as usize), petgraph::Direction::Incoming) {
            let source = edge.source().index() as u32;
            
            println!("    检查边 {} -> {} (边类型: {:?})", source, node, edge.weight().cfg_edge_type);
            
            // 检查是否是回边（基于支配关系）
            if self.is_back_edge(source, node, ctx) {
                println!("    ✅ 发现回边 {} -> {}", source, node);
                return true;
            } else {
                println!("    ❌ 边 {} -> {} 不是回边", source, node);
            }
        }
        
        println!("  ❌ 节点 {} 没有回边，不是循环头", node);
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
        
        println!("开始检查所有CFG节点...");
        
        // 先打印所有CFG边的信息
        println!("=== CFG边信息 ===");
        for edge_idx in ctx.cfg_graph.edge_indices() {
            let edge = ctx.cfg_graph.edge_endpoints(edge_idx).unwrap();
            let source = edge.0.index() as u32;
            let target = edge.1.index() as u32;
            let edge_weight = ctx.cfg_graph.edge_weight(edge_idx).unwrap();
            println!("边: {} -> {} (类型: {:?})", source, target, edge_weight.cfg_edge_type);
        }
        println!("=== CFG边信息结束 ===");
        
        // 遍历所有CFG节点，寻找循环头
        for node_idx in ctx.cfg_graph.node_indices() {
            let node = node_idx.index() as u32;
            
            if visited_headers.contains(&node) {
                continue;
            }
            
            println!("检查节点 {} 是否是循环头", node);
            
            if self.is_loop_header(node, ctx) {
                println!("✅ 发现规约循环头: {}", node);
                visited_headers.insert(node);
                
                // 识别循环信息
                let loop_info = self.identify_loop_from_header(node, ctx)?;
                loops.push(loop_info);
            } else {
                println!("❌ 节点 {} 不是循环头", node);
            }
        }
        
        println!("识别到 {} 个规约循环", loops.len());
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
        
        println!("开始识别循环头 {} 的循环体", header);
        
        // 添加循环头到循环体
        loop_body.insert(header);
        
        // 识别回边并收集循环体
        for edge in ctx.cfg_graph.edges_directed(petgraph::graph::NodeIndex::new(header as usize), petgraph::Direction::Incoming) {
            let source = edge.source().index() as u32;
            if self.is_back_edge(source, header, ctx) {
                back_edges.push((source, header));
                loop_body.insert(source);
                println!("发现回边: {} -> {}", source, header);
            }
        }
        
        // 简化的工作列表算法：只包含被循环头支配且在循环执行路径上的节点
        let mut worklist: Vec<u32> = loop_body.iter().cloned().collect();
        let mut processed = AHashSet::new();
        
        while let Some(node) = worklist.pop() {
            if processed.contains(&node) {
                continue;
            }
            processed.insert(node);
            
            // 只处理后继节点，确保循环体的连续性
            for edge in ctx.cfg_graph.edges(petgraph::graph::NodeIndex::new(node as usize)) {
                let succ = edge.target().index() as u32;
                
                if !loop_body.contains(&succ) {
                    // 检查后继是否被循环头支配
                    let succ_dj = match ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(succ as usize)).unwrap().get_cor_dj_node() {
                        std::result::Result::Ok(dj) => dj,
                        Err(_) => continue,
                    };
                    
                    let header_dj = match ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(header as usize)).unwrap().get_cor_dj_node() {
                        std::result::Result::Ok(dj) => dj,
                        Err(_) => continue,
                    };
                    
                    if self.is_dominator(*header_dj, *succ_dj, ctx) {
                        // 关键修复：检查边的类型，只有循环内部的边才包含节点
                        let edge_type = &edge.weight().cfg_edge_type;
                        let should_include = match edge_type {
                            CfgEdgeType::BodyHead { } => {
                                // BodyHead边表示循环内部关系
                                true
                            }
                            CfgEdgeType::BodyTail { } => {
                                // BodyTail边表示循环内部关系
                                true
                            }
                            CfgEdgeType::Direct { } => {
                                // Direct边表示顺序执行，不包含在循环体内
                                false
                            }
                            _ => {
                                // 其他类型的边也包含
                                true
                            }
                        };
                        
                        if should_include {
                            // 检查是否是其他循环头
                            let node_type = &ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(succ as usize)).unwrap().cfg_node_type;
                            if let CfgNodeType::WhileLoop { .. } = node_type {
                                // 如果是循环头，检查是否应该包含
                                // 只有当它是当前循环的直接子循环时才包含
                                let should_include_loop = self.is_direct_child_loop(succ, header, ctx);
                                if should_include_loop {
                                    println!("包含子循环头 {} 到循环体", succ);
                                    loop_body.insert(succ);
                                    worklist.push(succ);
                                } else {
                                    println!("排除其他循环头 {} 从循环体", succ);
                                }
                            } else {
                                // 普通节点，直接包含
                                println!("添加后继节点 {} 到循环体 (边类型: {:?})", succ, edge_type);
                                loop_body.insert(succ);
                                worklist.push(succ);
                            }
                        } else {
                            println!("排除节点 {} 从循环体 (边类型: {:?} 表示顺序执行)", succ, edge_type);
                        }
                    }
                }
            }
        }
        
        println!("循环头 {} 的循环体: {:?}", header, loop_body);
        
        // 识别循环出口
        for &node in &loop_body {
            for edge in ctx.cfg_graph.edges(petgraph::graph::NodeIndex::new(node as usize)) {
                let target = edge.target().index() as u32;
                if !loop_body.contains(&target) {
                    exits.push(target);
                }
            }
        }
        
        println!("循环头 {} 的出口: {:?}", header, exits);
        
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

    /// 检查是否是直接子循环
    fn is_direct_child_loop(&self, child_header: u32, parent_header: u32, ctx: &NhwcCtx) -> bool {
        // 检查支配关系
        let child_dj = match ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(child_header as usize)).unwrap().get_cor_dj_node() {
            std::result::Result::Ok(dj) => dj,
            Err(_) => return false,
        };
        
        let parent_dj = match ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(parent_header as usize)).unwrap().get_cor_dj_node() {
            std::result::Result::Ok(dj) => dj,
            Err(_) => return false,
        };
        
        if !self.is_dominator(*parent_dj, *child_dj, ctx) {
            return false;
        }
        
        // 检查是否有直接的CFG边连接
        for edge in ctx.cfg_graph.edges(petgraph::graph::NodeIndex::new(parent_header as usize)) {
            let target = edge.target().index() as u32;
            if target == child_header {
                // 检查边的类型
                match edge.weight().cfg_edge_type {
                    CfgEdgeType::Direct { } => {
                        // Direct边表示顺序执行，不是嵌套关系
                        return false;
                    }
                    CfgEdgeType::BodyHead { } => {
                        // BodyHead边表示嵌套关系
                        return true;
                    }
                    _ => {
                        // 其他类型的边也允许嵌套
                        return true;
                    }
                }
            }
        }
        
        false
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
        // 改进实现：只检查节点是否是其他循环头，而不是被其他循环头支配
        // 这样可以避免误判循环体中的普通节点
        for node_idx in ctx.cfg_graph.node_indices() {
            let potential_header = node_idx.index() as u32;
            if potential_header == exclude_header {
                continue;
            }
            
            // 只检查节点本身是否是循环头
            if node == potential_header && self.is_loop_header(potential_header, ctx) {
                return true;
            }
        }
        
        false
    }

    /// 获取循环体（简化版本，用于嵌套关系判断）
    fn get_loop_body(&self, header: u32, ctx: &NhwcCtx) -> AHashSet<u32> {
        let mut loop_body = AHashSet::new();
        loop_body.insert(header);
        
        // 识别回边并收集循环体
        for edge in ctx.cfg_graph.edges_directed(petgraph::graph::NodeIndex::new(header as usize), petgraph::Direction::Incoming) {
            let source = edge.source().index() as u32;
            if self.is_back_edge(source, header, ctx) {
                loop_body.insert(source);
            }
        }
        
        // 使用工作列表算法扩展循环体
        let mut worklist: Vec<u32> = loop_body.iter().cloned().collect();
        let mut processed = AHashSet::new();
        
        while let Some(node) = worklist.pop() {
            if processed.contains(&node) {
                continue;
            }
            processed.insert(node);
            
            // 处理前驱节点
            for edge in ctx.cfg_graph.edges_directed(petgraph::graph::NodeIndex::new(node as usize), petgraph::Direction::Incoming) {
                let pred = edge.source().index() as u32;
                
                if !loop_body.contains(&pred) {
                    let pred_dj = match ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(pred as usize)).unwrap().get_cor_dj_node() {
                        std::result::Result::Ok(dj) => dj,
                        Err(_) => continue,
                    };
                    
                    let header_dj = match ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(header as usize)).unwrap().get_cor_dj_node() {
                        std::result::Result::Ok(dj) => dj,
                        Err(_) => continue,
                    };
                    
                    if self.is_dominator(*header_dj, *pred_dj, ctx) && self.is_reachable_from(header, pred, ctx) {
                        loop_body.insert(pred);
                        worklist.push(pred);
                    }
                }
            }
            
            // 处理后继节点
            for edge in ctx.cfg_graph.edges(petgraph::graph::NodeIndex::new(node as usize)) {
                let succ = edge.target().index() as u32;
                
                if !loop_body.contains(&succ) {
                    let succ_dj = match ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(succ as usize)).unwrap().get_cor_dj_node() {
                        std::result::Result::Ok(dj) => dj,
                        Err(_) => continue,
                    };
                    
                    let header_dj = match ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(header as usize)).unwrap().get_cor_dj_node() {
                        std::result::Result::Ok(dj) => dj,
                        Err(_) => continue,
                    };
                    
                    if self.is_dominator(*header_dj, *succ_dj, ctx) {
                        loop_body.insert(succ);
                        worklist.push(succ);
                    }
                }
            }
        }
        
        loop_body
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
        
        println!("循环头排序结果: {:?}", loop_headers);
        
        // 构建嵌套关系 - 完全重写版本，基于CFG结构
        for i in 0..loop_headers.len() {
            let header = loop_headers[i];
            let loop_info = loops[&header].clone();
            
            println!("检查循环 {} 的嵌套关系", header);
            
            // 寻找直接父循环 - 基于CFG边的连接关系
            let mut best_parent = None;
            let mut best_depth = 0;
            
            for j in (0..i).rev() {
                let potential_parent = loop_headers[j];
                let parent_loop = &loops[&potential_parent];
                
                // 检查是否有CFG边从父循环头直接连接到子循环头
                let mut has_direct_edge = false;
                for edge in ctx.cfg_graph.edges(petgraph::graph::NodeIndex::new(potential_parent as usize)) {
                    let target = edge.target().index() as u32;
                    if target == header {
                        has_direct_edge = true;
                        println!("发现从循环 {} 到循环 {} 的直接CFG边", potential_parent, header);
                        break;
                    }
                }
                
                // 关键修复：检查边的类型
                if has_direct_edge {
                    for edge in ctx.cfg_graph.edges(petgraph::graph::NodeIndex::new(potential_parent as usize)) {
                        let target = edge.target().index() as u32;
                        if target == header {
                            match edge.weight().cfg_edge_type {
                                CfgEdgeType::Direct { } => {
                                    println!("发现顺序执行关系：循环 {} 通过Direct边连接到循环 {}，不嵌套", potential_parent, header);
                                    continue; // 跳过这个潜在父循环
                                }
                                CfgEdgeType::BodyHead { } => {
                                    println!("发现嵌套关系：循环 {} 通过BodyHead边连接到循环 {}", potential_parent, header);
                                }
                                _ => {
                                    // 其他类型的边也允许嵌套
                                }
                            }
                        }
                    }
                }
                
                // 检查支配关系
                let child_dj = match ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(header as usize)).unwrap().get_cor_dj_node() {
                    std::result::Result::Ok(dj) => dj,
                    Err(_) => continue,
                };
                
                let parent_dj = match ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(potential_parent as usize)).unwrap().get_cor_dj_node() {
                    std::result::Result::Ok(dj) => dj,
                    Err(_) => continue,
                };
                
                if self.is_dominator(*parent_dj, *child_dj, ctx) {
                    // 如果有直接CFG边连接，检查是否是顺序执行关系
                    if has_direct_edge {
                        // 检查边的类型
                        let mut is_sequential = false;
                        for edge in ctx.cfg_graph.edges(petgraph::graph::NodeIndex::new(potential_parent as usize)) {
                            let target = edge.target().index() as u32;
                            if target == header {
                                match edge.weight().cfg_edge_type {
                                    CfgEdgeType::Direct { } => {
                                        is_sequential = true;
                                        break;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        
                        if is_sequential {
                            println!("循环 {} 和循环 {} 是顺序执行关系，不嵌套", header, potential_parent);
                            continue; // 跳过这个潜在父循环
                        }
                    }
                    
                                    // 检查循环体包含关系
                let parent_loop_body = self.get_loop_body(potential_parent, ctx);
                if parent_loop_body.contains(&header) {
                    println!("循环 {} 被循环 {} 支配且在循环体内，确认嵌套关系", header, potential_parent);
                    
                    // 关键修复：优先选择有直接CFG边连接的父循环
                    let has_direct_edge = ctx.cfg_graph.edges(petgraph::graph::NodeIndex::new(potential_parent as usize))
                        .any(|edge| edge.target().index() as u32 == header);
                    
                    if has_direct_edge {
                        // 检查边的类型，确保是嵌套关系而不是顺序执行关系
                        let mut is_nesting_edge = false;
                        for edge in ctx.cfg_graph.edges(petgraph::graph::NodeIndex::new(potential_parent as usize)) {
                            let target = edge.target().index() as u32;
                            if target == header {
                                match edge.weight().cfg_edge_type {
                                    CfgEdgeType::BodyHead { } => {
                                        is_nesting_edge = true;
                                        break;
                                    }
                                    CfgEdgeType::Direct { } => {
                                        // Direct边表示顺序执行，不是嵌套关系
                                        is_nesting_edge = false;
                                        break;
                                    }
                                    _ => {
                                        // 其他类型的边也允许嵌套
                                        is_nesting_edge = true;
                                        break;
                                    }
                                }
                            }
                        }
                        
                        if is_nesting_edge {
                            // 如果有直接CFG边连接且是嵌套关系，优先选择这个父循环
                            best_parent = Some(potential_parent);
                            best_depth = parent_loop.depth;
                            println!("找到直接嵌套连接的父循环: {} (深度: {})", potential_parent, best_depth);
                            break; // 直接选择这个父循环，不再继续查找
                        } else {
                            println!("循环 {} 和循环 {} 有Direct边连接，是顺序执行关系，不嵌套", header, potential_parent);
                        }
                    } else {
                        // 关键修复：检查是否有更直接的嵌套关系
                        // 遍历所有其他循环，看是否有更直接的嵌套关系
                        let mut has_more_direct_nesting = false;
                        for (other_header, other_loop) in loops.iter() {
                            if *other_header != potential_parent && *other_header != header {
                                // 检查当前循环是否被其他循环嵌套，且其他循环被当前潜在父循环嵌套
                                if other_loop.body.contains(&header) && parent_loop_body.contains(other_header) {
                                    has_more_direct_nesting = true;
                                    println!("发现更直接的嵌套关系：循环 {} 被循环 {} 嵌套，循环 {} 被循环 {} 嵌套", 
                                        header, other_header, other_header, potential_parent);
                                    break;
                                }
                            }
                        }
                        
                        if !has_more_direct_nesting && parent_loop.depth >= best_depth {
                            best_parent = Some(potential_parent);
                            best_depth = parent_loop.depth;
                            println!("找到更好的父循环: {} (深度: {})", potential_parent, best_depth);
                        }
                    }
                }
                }
            }
            
            // 设置嵌套关系
            if let Some(parent_header) = best_parent {
                loops.get_mut(&header).unwrap().parent = Some(parent_header);
                loops.get_mut(&parent_header).unwrap().children.push(header);
                println!("设置嵌套关系：循环 {} 嵌套在循环 {} 内部", header, parent_header);
            } else {
                println!("循环 {} 没有找到父循环，是根循环", header);
            }
        }
        
        // 计算嵌套深度
        println!("开始计算循环嵌套深度...");
        
        // 先初始化所有循环的深度为0
        for loop_info in loops.values_mut() {
            loop_info.depth = 0;
        }
        
        // 只从根循环开始计算深度，避免重复计算
        for &header in &loop_headers {
            if loops[&header].parent.is_none() {
                println!("从根循环 {} 开始计算嵌套深度", header);
                self.compute_loop_depth(header, loops, 0);
            }
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
        
        if !self.is_dominator(*parent_dj, *child_dj, ctx) {
            return false;
        }
        
        // 3. 检查循环执行顺序 - 关键修改
        // 如果两个循环是顺序执行的（有直接的CFG边连接），则不应该嵌套
        for &parent_node in &parent.body {
            for edge in ctx.cfg_graph.edges(petgraph::graph::NodeIndex::new(parent_node as usize)) {
                let target = edge.target().index() as u32;
                if target == child.header {
                    // 如果父循环体中的节点直接连接到子循环头，说明它们是顺序执行的
                    println!("发现顺序执行关系：父循环 {} 的节点 {} 直接连接到子循环头 {}", 
                        parent.header, parent_node, child.header);
                    return false;
                }
            }
        }
        
        // 4. 检查是否有其他循环头在支配路径上
        // 如果子循环头被父循环头直接支配，且没有其他循环头在它们之间
        let mut current = child.header;
        let mut found_parent = false;
        
        // 沿着支配树向上查找，看是否直接连接到父循环头
        while current != parent.header {
            let current_dj = match ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(current as usize)).unwrap().get_cor_dj_node() {
                std::result::Result::Ok(dj) => dj,
                Err(_) => return false,
            };
            
            // 获取当前节点的父节点（在支配树中）
            let parent_nodes: Vec<_> = ctx.dj_graph.edges_directed(petgraph::graph::NodeIndex::new(*current_dj as usize), petgraph::Direction::Incoming)
                .filter(|e| e.weight().is_dom())
                .map(|e| e.source().index() as u32)
                .collect();
            
            let parent_node = match parent_nodes.first() {
                Some(&parent) => parent,
                None => return false,
            };
            
            // 检查这个父节点是否是其他循环头
            // 遍历所有CFG节点，检查是否有其他循环头
            for node_idx in ctx.cfg_graph.node_indices() {
                let potential_header = node_idx.index() as u32;
                if potential_header != parent.header && potential_header != child.header {
                    // 检查是否是循环头
                    if self.is_loop_header(potential_header, ctx) {
                        let other_dj = match ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(potential_header as usize)).unwrap().get_cor_dj_node() {
                            std::result::Result::Ok(dj) => dj,
                            Err(_) => continue,
                        };
                        
                        if *other_dj == parent_node {
                            // 发现中间有其他循环头，说明不是直接嵌套
                            println!("发现中间循环头：{} 在 {} 和 {} 之间", potential_header, parent.header, child.header);
                            return false;
                        }
                    }
                }
            }
            
            if parent_node == current {
                return false; // 防止循环
            }
            
            current = parent_node;
            found_parent = true;
        }
        
        found_parent
    }

    /// 改进的循环嵌套关系检查 - 专门用于construct_loop_tree
    fn should_nest_improved(&self, child_header: u32, parent_header: u32, ctx: &NhwcCtx) -> bool {
        // 简化的嵌套关系判断逻辑
        
        // 1. 检查支配关系
        let child_dj = match ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(child_header as usize)).unwrap().get_cor_dj_node() {
            std::result::Result::Ok(dj) => dj,
            Err(_) => return false,
        };
        
        let parent_dj = match ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(parent_header as usize)).unwrap().get_cor_dj_node() {
            std::result::Result::Ok(dj) => dj,
            Err(_) => return false,
        };
        
        if !self.is_dominator(*parent_dj, *child_dj, ctx) {
            return false;
        }
        
        // 2. 关键修复：检查是否有直接的CFG边连接（顺序执行关系）
        // 检查从父循环头到子循环头的直接边
        for edge in ctx.cfg_graph.edges(petgraph::graph::NodeIndex::new(parent_header as usize)) {
            let target = edge.target().index() as u32;
            if target == child_header {
                // 检查边的类型
                match edge.weight().cfg_edge_type {
                    CfgEdgeType::Direct { } => {
                        println!("发现顺序执行关系：父循环头 {} 直接连接到子循环头 {} (Direct边)", parent_header, child_header);
                        return false;
                    }
                    CfgEdgeType::BodyHead { } => {
                        // BodyHead边是正常的嵌套关系，不阻止嵌套
                        println!("发现嵌套关系：父循环头 {} 通过BodyHead边连接到子循环头 {}", parent_header, child_header);
                    }
                    _ => {
                        // 其他类型的边也不阻止嵌套
                    }
                }
            }
        }
        
        // 3. 检查父循环体中的节点是否直接连接到子循环头
        // 获取父循环的循环体
        let parent_loop_body = self.get_loop_body(parent_header, ctx);
        
        for &parent_node in &parent_loop_body {
            if parent_node != parent_header && parent_node != child_header {
                // 检查这个节点是否直接连接到子循环头
                for edge in ctx.cfg_graph.edges(petgraph::graph::NodeIndex::new(parent_node as usize)) {
                    let target = edge.target().index() as u32;
                    if target == child_header {
                        // 检查边的类型，只有Direct边才表示顺序执行关系
                        match edge.weight().cfg_edge_type {
                            CfgEdgeType::Direct { } => {
                                println!("发现顺序执行关系：父循环 {} 的节点 {} 通过Direct边连接到子循环头 {}", 
                                    parent_header, parent_node, child_header);
                                return false;
                            }
                            CfgEdgeType::BodyHead { } => {
                                // BodyHead边是正常的循环内部关系，不阻止嵌套
                                println!("发现循环内部关系：父循环 {} 的节点 {} 通过BodyHead边连接到子循环头 {}", 
                                    parent_header, parent_node, child_header);
                            }
                            _ => {
                                // 其他类型的边也不阻止嵌套
                            }
                        }
                    }
                }
            }
        }
        
        // 4. 检查循环体包含关系
        if parent_loop_body.contains(&child_header) {
            println!("子循环头 {} 在父循环 {} 的循环体内，确认嵌套关系", child_header, parent_header);
            return true;
        }
        
        // 5. 如果没有明确的嵌套关系，则不嵌套
        println!("循环 {} 和循环 {} 没有明确的嵌套关系", child_header, parent_header);
        false
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
            println!("设置循环 {} 的深度为 {}", header, depth);
            
            // 递归计算子循环深度
            for &child_header in &loop_info.children.clone() {
                println!("递归计算子循环 {} 的深度", child_header);
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
            
            // 使用支配树遍历CFG，优化优先级以确保循环按正确顺序处理
            let dfs_nodes = dfs_with_priority(&ctx.cfg_graph, cfg_entry, |e| match &e.weight().cfg_edge_type {
                CfgEdgeType::BodyHead { } => 1,  // 优先处理循环头
                CfgEdgeType::IfTrue { } => 1,    // 优先处理true分支
                CfgEdgeType::Direct { } => 2,    // 顺序执行
                CfgEdgeType::IfFalse { } => 3,   // 后处理false分支
                CfgEdgeType::BodyTail { } => -1, // 最后处理循环尾
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
                    } else {
                        // 如果没有明确的父循环，检查是否有嵌套关系
                        // 基于支配关系和CFG结构判断嵌套
                        for (potential_parent_header, potential_parent_loop) in loops {
                            if *potential_parent_header != node_idx {
                                // 检查当前循环头是否被潜在父循环头支配
                                let current_dj = match ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(node_idx as usize)).unwrap().get_cor_dj_node() {
                                    std::result::Result::Ok(dj) => dj,
                                    Err(_) => continue,
                                };
                                
                                let parent_dj = match ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(*potential_parent_header as usize)).unwrap().get_cor_dj_node() {
                                    std::result::Result::Ok(dj) => dj,
                                    Err(_) => continue,
                                };
                                
                                if self.is_dominator(*parent_dj, *current_dj, ctx) {
                                    // 检查是否应该嵌套 - 使用改进的逻辑
                                    let should_nest = self.should_nest_improved(node_idx, *potential_parent_header, ctx);
                                    
                                    if should_nest {
                                        // 找到嵌套关系
                                        if let Some(parent_loop_idx) = node_to_tree_node.get(potential_parent_header) {
                                            parent_node = *parent_loop_idx;
                                            println!("发现嵌套关系：循环 {} 嵌套在循环 {} 内部 (改进的判断)", node_idx, potential_parent_header);
                                            break;
                                        }
                                    } else {
                                        println!("循环 {} 和循环 {} 是顺序执行关系，不嵌套", node_idx, potential_parent_header);
                                    }
                                }
                            }
                        }
                    }
                    
                    let loop_type_str = if loop_info.is_reducible() { "规约" } else { "不可规约" };
                    println!("{}循环头 {} 的循环节点挂在父节点 {:?} 下", loop_type_str, node_idx, parent_node);
                    tree.add_edge(parent_node, loop_idx, ());
                    node_to_tree_node.insert(node_idx, loop_idx);
                    
                    // 添加循环体中的基本块节点（不包括循环头）
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
                                
                                // 检查节点是否是其他循环头，如果是则跳过
                                if loops.contains_key(&body_node) {
                                    println!("跳过循环体中的其他循环头节点: {}", body_node);
                                    continue;
                                }
                                
                                // 关键修复：检查节点是否真正属于当前循环
                                // 基于CFG结构和支配关系判断
                                let mut should_add_to_current_loop = true;
                                
                                // 检查节点是否被当前循环头支配
                                let body_node_dj = match ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(body_node as usize)).unwrap().get_cor_dj_node() {
                                    std::result::Result::Ok(dj) => dj,
                                    Err(_) => {
                                        println!("跳过节点 {} (无法获取DJ节点)", body_node);
                                        continue;
                                    }
                                };
                                
                                let header_dj = match ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(node_idx as usize)).unwrap().get_cor_dj_node() {
                                    std::result::Result::Ok(dj) => dj,
                                    Err(_) => {
                                        println!("跳过节点 {} (无法获取循环头DJ节点)", body_node);
                                        continue;
                                    }
                                };
                                
                                if !self.is_dominator(*header_dj, *body_node_dj, ctx) {
                                    should_add_to_current_loop = false;
                                    println!("节点 {} 不被循环头 {} 支配，不添加到当前循环", body_node, node_idx);
                                }
                                
                                // 检查节点是否属于其他更内层循环
                                for (other_header, other_loop_info) in loops {
                                    if *other_header != node_idx && other_loop_info.body.contains(&body_node) {
                                        // 检查其他循环是否嵌套在当前循环内部
                                        if let Some(parent) = other_loop_info.parent {
                                            if parent == node_idx {
                                                should_add_to_current_loop = false;
                                                println!("节点 {} 属于嵌套在当前循环 {} 内部的循环 {}，不添加到当前循环", body_node, node_idx, other_header);
                                                break;
                                            }
                                        }
                                    }
                                }
                                
                                // 关键修复：检查节点是否真正属于当前循环
                                // 基于CFG结构和支配关系进行精确判断
                                if should_add_to_current_loop {
                                    // 检查节点是否被当前循环头直接支配
                                    let body_node_dj = match ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(body_node as usize)).unwrap().get_cor_dj_node() {
                                        std::result::Result::Ok(dj) => dj,
                                        Err(_) => {
                                            should_add_to_current_loop = false;
                                            continue;
                                        }
                                    };
                                    
                                    let header_dj = match ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(node_idx as usize)).unwrap().get_cor_dj_node() {
                                        std::result::Result::Ok(dj) => dj,
                                        Err(_) => {
                                            should_add_to_current_loop = false;
                                            continue;
                                        }
                                    };
                                    
                                    if !self.is_dominator(*header_dj, *body_node_dj, ctx) {
                                        should_add_to_current_loop = false;
                                        println!("节点 {} 不被循环头 {} 支配，不添加到当前循环", body_node, node_idx);
                                    }
                                    
                                    // 检查节点是否在循环的执行路径上
                                    if should_add_to_current_loop && !self.is_reachable_from(node_idx, body_node, ctx) {
                                        should_add_to_current_loop = false;
                                        println!("节点 {} 从循环头 {} 不可达，不添加到当前循环", body_node, node_idx);
                                    }
                                    
                                    // 关键修复：检查节点是否属于更内层的循环
                                    if should_add_to_current_loop {
                                        for (other_header, other_loop_info) in loops {
                                            if *other_header != node_idx && other_loop_info.body.contains(&body_node) {
                                                // 检查其他循环是否嵌套在当前循环内部
                                                if let Some(parent) = other_loop_info.parent {
                                                    if parent == node_idx {
                                                        // 如果节点属于嵌套在当前循环内部的循环，则不添加到当前循环
                                                        should_add_to_current_loop = false;
                                                        println!("节点 {} 属于嵌套循环 {}，不添加到当前循环 {}", body_node, other_header, node_idx);
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                
                                if should_add_to_current_loop {
                                    let body_node_type = ctx.cfg_graph.node_weight(petgraph::graph::NodeIndex::new(body_node as usize)).unwrap().cfg_node_type.clone();
                                    let body_tree_node = tree.add_node(LoopNode::new_terminal_node(body_node, body_node_type));
                                    tree.add_edge(loop_idx, body_tree_node, ());
                                    node_to_tree_node.insert(body_node, body_tree_node);
                                    println!("添加循环体节点: {} -> 树节点: {:?} (挂在当前循环 {:?} 下)", body_node, body_tree_node, loop_idx);
                                }
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
