use petgraph::GraphMap;
use rejson::ReJson;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
struct Node {
    label: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
struct Edge {
    weight: f32,
}

fn main() {
    let mut graph = GraphMap::<Node, Edge>::new();
    let node1 = graph.add_node(Node { label: "A" });
    let node2 = graph.add_node(Node { label: "B" });
    let node3 = graph.add_node(Node { label: "C" });
    graph.add_edge(node1, node2, Edge { weight: 1.0 });
    graph.add_edge(node2, node3, Edge { weight: 2.0 });

    let rejson = ReJson::new();
    let serialized_graph = rejson.serialize(&graph).unwrap();
    let deserialized_graph: GraphMap<Node, Edge> = rejson.deserialize(&serialized_graph).unwrap();

    assert_eq!(graph, deserialized_graph);
}
