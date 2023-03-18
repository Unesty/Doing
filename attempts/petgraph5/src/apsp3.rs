use petgraph::{Graph, Undirected};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

type MyGraph = Graph<String, (), Undirected>;

#[derive(Serialize, Deserialize)]
struct SerializedGraph {
    node_count: usize,
    nodes: Vec<String>,
    edges: Vec<(usize, usize)>,
}

fn save_graph(graph: &MyGraph, path: &str) -> std::io::Result<()> {
    let serialized_graph = SerializedGraph {
        node_count: graph.node_count(),
        nodes: graph
            .node_indices()
            .map(|i| graph[i].clone())
            .collect::<Vec<String>>(),
        edges: graph
            .edge_indices()
            .map(|e| {
                let (source, target) = graph.edge_endpoints(e).unwrap();
                (source.index(), target.index())
            })
            .collect::<Vec<(usize, usize)>>(),
    };
    let serialized_graph_str = serde_json::to_string(&serialized_graph).unwrap();

    let mut file = File::create(path)?;
    file.write_all(serialized_graph_str.as_bytes())?;

    Ok(())
}

fn load_graph(path: &str) -> std::io::Result<MyGraph> {
    let mut file = File::open(path)?;
    let mut serialized_graph_str = String::new();
    file.read_to_string(&mut serialized_graph_str)?;

    let serialized_graph: SerializedGraph = serde_json::from_str(&serialized_graph_str).unwrap();

    let mut graph = MyGraph::new_undirected();
    for node in serialized_graph.nodes {
        graph.add_node(node);
    }
    for (source, target) in serialized_graph.edges {
        graph.add_edge(NodeIndex::from(source), graph.node_indices(target), ());
    }

    Ok(graph)
}

fn main() {
    let mut graph = MyGraph::new_undirected();
    let a = graph.add_node("a".to_owned());
    let b = graph.add_node("b".to_owned());
    let c = graph.add_node("c".to_owned());
    graph.add_edge(a, b, ());
    graph.add_edge(b, c, ());
    graph.add_edge(c, a, ());

    save_graph(&graph, "mygraph.json").unwrap();

    let loaded_graph = load_graph("mygraph.json").unwrap();
    assert_eq!(loaded_graph.node_count(), 3);
}
