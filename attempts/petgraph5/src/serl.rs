// use petgraph::graph::Graph;
// use std::fs::write;
// use std::fs::read;
// // use petgraph::dot::{Dot, Config};
//
// fn main() {
//     let mut dwgraph = Graph::<&str, u32>::new();
//     // let grtxt = read("graph.dot");
//     // dwgraph = Dot::parse_dot;
//
// }

use petgraph::matrix_graph::NodeIndex;
use petgraph::EdgeType;
use petgraph::Graph;
use serde::{Deserialize, Serialize};
use std::{fs::File};
use petgraph::algo::dijkstra;

#[derive(Serialize, Deserialize)]
struct NodeData {
    name: String,
    value: u32,
}

fn main() {
    // Read graph from file
    let file = File::open("graph.json").expect("Failed to open file");
    let mut graph: Graph<NodeData, u32> = serde_json::from_reader(file).expect("Failed to parse JSON");

    // Modify graph
    let modified_graph = modify_graph(graph);

    // Write modified graph to file
    let file = File::create("modified_graph.json").expect("Failed to create file");
    serde_json::to_writer_pretty(file, &modified_graph).expect("Failed to write JSON");
}

// fn modify_graph(mut graph: Graph<NodeData, u32>) -> Graph<NodeData, u32> {
//     // Find shortest path between two nodes
//     let start = graph.node_indices().next().unwrap();
//     let end = graph.node_indices().nth(1).unwrap();
//     let path = dijkstra(&graph, start, Some(end), |_| 1);
//
//     // Create new graph with only nodes in the shortest path
//     let mut new_graph: Graph<NodeData, u32> = Graph::new();
//     // let mut node_map = Vec::new();
//     for node in path {
//         let node_data: NodeData;
//         // node_data.name = *graph.node_weight(node.0).name;
//         node_data = graph.filter_map(path);
//         let new_node = new_graph.add_node(node_data);
//         // let new_node = new_graph.add_node(node);
//         // node_map.push((node, new_node));
//
//     }
//     // for edge in graph.edge_indices() {
//     //     let (src, dst) = graph.edge_endpoints(edge).unwrap();
//     //     let (new_src, new_dst) = (node_map[src.index()].1, node_map[dst.index()].1);
//     //     new_graph.add_edge(new_src, new_dst, *graph.edge_weight(edge).unwrap());
//     // }
//
//
//     new_graph
// }
/*
fn modify_graph(graph: Graph<NodeData, u32>)  -> Graph<NodeData, u32> {
    // Find shortest path between two nodes
    let start = graph.node_indices().next().unwrap();
    let end = graph.node_indices().nth(1).unwrap();
    let path = dijkstra(&graph, start, Some(end), |_| 1);

    let mut nodes_to_connect: Vec<NodeIndex> = Vec::new();

    // Copy nodes along the shortest path
    for node in path.into_iter().skip(1).rev() {
        let node_data = graph.node_weight(node).unwrap().clone();
        let new_node = graph.add_node(node_data.clone());
        nodes_to_connect.push(node);
        nodes_to_connect.push(new_node);
    }

    // Connect copied nodes to original nodes using edge type "same"
    for pair in nodes_to_connect.windows(2) {
        let src = pair[0];
        let dst = pair[1];
        let edge_weight = match graph.find_edge(src, dst) {
            Some(edge) => *graph.edge_weight(edge).unwrap(),
            None => *graph.edge_weight(dst, src).unwrap(),
        };
        graph.add_edge(src, dst, EdgeType::Same(edge_weight));
    }
    graph
}*/

fn link_graphs(
    graph1: &mut Graph<i32, (), petgraph::Undirected>,
    graph2: &mut Graph<i32, (), petgraph::Undirected>,
    node_mapping: &std::collections::HashMap<usize, petgraph::graph::NodeIndex<u32>>,
    source_node: usize,
    target_node: usize,
) {
    // Get the node indices from the hash map
    let source_index = *node_mapping.get(&source_node).unwrap();
    let target_index = *node_mapping.get(&target_node).unwrap();

    // Create a new edge between the two nodes in the two graphs
    graph1.add_edge(source_index, target_index, ());
    graph2.add_edge(target_index, source_index, ());
}

// In this function we should find shortest path among edges with "PrTr" type, create new node in chain of graph rewrite nodes,
// connect in to root node of represented graph,
// add nodes of shortest path to another subgraph,
// connect copied nodes with origin nodes using "same" edge type.
// Return modified graph.
// fn add_optimized_graph(mut graph: Graph<NodeData, u32>, mut rewrite_id: u8) -> Graph<NodeData, u32> {
// }

// So it will be like so:
// let mut graph = Graph::<NodeData, u32>::new();
// let mut data = collected_data();
// let mut i = 0;
// graph = add_optimized_graph(graph, i);
// memory, encoder, decoder, pathfinder, pathfollower are defined inside 1 graph, so all of them are in 1 memory region
// let mut optimized_mem_enc_dec_opt = compile_to_4bitproc(graph);
// let mut graphmem = get_graphmem(optimized_mem_enc_dec_opt);
// let mut graphenc = get_graphenc(optimized_mem_enc_dec_opt);
// let mut graphdec = get_graphdec(optimized_mem_enc_dec_opt);
// optimizer executes pathfinder, then executes pathfollower
// let mut graphopt = get_graphopt(optimized_mem_enc_dec_opt);
// maybe, optimizer will find the way that doesn't contain function enters, exits, memory,
// so everything will be executed at once in a single loop iteration
// while(true) {
//     i+=1;
//
// }
