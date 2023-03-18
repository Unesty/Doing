use petgraph::matrix_graph::NodeIndex;
use petgraph::EdgeType;
use petgraph::Graph;
use serde::{Deserialize, Serialize};
use std::{fs::File};
use petgraph::algo::dijkstra;
use rand;
use rand::Rng;


#[derive(Debug, Clone, Serialize, Deserialize)]
struct Transition {
    current_state: u16,
    next_state: u16,
    count: u32,
}
fn main() {
    let mut graph = Graph::<u16, Transition>::new();
    for i in 0..65535 {
        graph.add_node(i);
    }

    let mut rng = rand::thread_rng();
    for i in 0..65535 {
        let current_state = i;
        let next_state = rng.gen_range(0..65535);
        let count = 1;
        let edge = graph.find_edge(NodeIndex::new(current_state as usize), NodeIndex::new(next_state as usize));
        match edge {
            Some(edge) => {
                let weight = graph.edge_weight_mut(edge).unwrap();
                weight.count += 1;
            },
            None => {
                let transition = Transition { current_state, next_state, count };
                graph.add_edge(NodeIndex::new(current_state as usize), NodeIndex::new(next_state as usize), transition);
            },
        }
    }
    let graph_json = serde_json::to_string(&graph).unwrap();
    std::fs::write("rndstates_graph.json", graph_json).unwrap();
}
