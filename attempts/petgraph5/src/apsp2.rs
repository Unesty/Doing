use petgraph::Graph;
use petgraph::prelude::*;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use petgraph::algo::dijkstra;

#[derive(Serialize, Deserialize)]
struct GraphData {
    nodes: Vec<i32>,
    edges: Vec<(usize, usize, i32)>,
}
fn create_graph() -> MyGraph {
    let mut graph = MyGraph::new();
    let node_a = graph.add_node(1);
    let node_b = graph.add_node(2);
    let node_c = graph.add_node(3);
    graph.add_edge(node_a, node_b, 10);
    graph.add_edge(node_a, node_c, 20);
    graph
}

type MyGraph = Graph<i32, i32>;

pub trait GraphIO {
    fn save(&self, path: &str) -> Result<(), Box<dyn Error>>;
    fn load(path: &str) -> Result<Self, Box<dyn Error>> where Self: Sized;
}

impl GraphIO for MyGraph {
    fn save(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(path)?;
        let mut nodes = Vec::new();
        for node in self.node_indices() {
            nodes.push((node.index(), *self.node_weight(node).unwrap()));
        }
        let mut edges = Vec::new();
        for edge in self.edge_indices() {
            let (source, target) = self.edge_endpoints(edge).unwrap();
            edges.push((source.index(), target.index(), *self.edge_weight(edge).unwrap()));
        }
        let data = serde_json::to_string(&(nodes, edges))?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        let mut file = File::open(path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let (nodes, edges): (Vec<(usize, i32)>, Vec<(usize, usize, i32)>) = serde_json::from_str(&data)?;
        let mut graph = MyGraph::new();
        let mut node_map = Vec::with_capacity(nodes.len());
        for (index, weight) in nodes {
            let node = graph.add_node(weight);
            node_map.push((index, node));
        }
        for (source, target, weight) in edges {
            let edge = graph.add_edge(node_map[source].1, node_map[target].1, weight);
        }
        Ok(graph)
    }
}

fn main() {
    let graph = create_graph();
    graph.save("mygraph.json").unwrap();
    let loaded_graph = MyGraph::load("mygraph.json").unwrap();
    let grhashmap = dijkstra(&graph, NodeIndex::from(0), None, |e| *e.weight());
    match grhashmap {
        Some((cost, path)) => {
            println!("The total cost was {}: {:?}", cost, path);
        }
        None => println!("There was no path"),
    }
}
