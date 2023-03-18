use petgraph::graph::Graph;
use std::fs::write;
use std::fs::File;
use petgraph::dot::{Dot, Config};

fn main() {
    let mut graph = Graph::<&str, u32>::new();
    let origin = graph.add_node("0");
    let destination_1 = graph.add_node("1");
    let destination_2 = graph.add_node("1");

    graph.extend_with_edges(&[
        (origin, destination_1, 4),
        (origin, destination_2, 6)
    ]);
    let grjson = serde_json::to_string(&graph).unwrap();

    let grparsed: Graph<&str, u32> = serde_json::from_str(&grjson).unwrap();
    println!("{}", grjson);
    // Save to file, then load here

    // Verify that saved and loaded graphs are the same
    assert_eq!(graph.node_count(), grparsed.node_count());
    assert_eq!(graph.edge_count(), grparsed.edge_count());

    for node in graph.node_indices() {
        assert_eq!(graph[node], grparsed[node]);
    }

    for edge in graph.edge_indices() {
        let (source, target) = graph.edge_endpoints(edge).unwrap();
        let (parsed_source, parsed_target) = grparsed.edge_endpoints(edge).unwrap();
        assert_eq!(graph[edge], grparsed[edge]);
        assert_eq!(source, parsed_source);
        assert_eq!(target, parsed_target);
    }
    // Write modified graph to file
    let file = File::create("graph.json").expect("Failed to create file");
    serde_json::to_writer(file, &graph).expect("Failed to write JSON");
    // Then we should use DOT instead to modify graph easily

    // Generate DOT code
    let dot_code = format!("{}", Dot::new(&graph));

    // Write DOT code to file
    write("graph.dot", dot_code).unwrap();

    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    // After finding out how to read/write graph, we need to use BFS to find
    // shortest path in graph of all possible states of computer that finds
    // all possible implementations of APSP.
}
