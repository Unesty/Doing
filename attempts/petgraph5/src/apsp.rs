use petgraph::{algo, prelude::*}; // 0.5.1

fn main() {
    let mut graph = Graph::new();

    // 
    let a = graph.add_node("explorer");
    let b = graph.add_node("");
    let c = graph.add_node("c");
    let d = graph.add_node("d");

    graph.extend_with_edges(&[(a, b, 1), (b, c, 1), (c, d, 1), (a, b, 1), (b, d, 1)]);

    let path = algo::astar(
        &graph,
        a,               // start
        |n| n == d,      // is_goal
        |e| *e.weight(), // edge_cost
        |_| 0,           // estimate_cost
    );

    match path {
        Some((cost, path)) => {
            println!("The total cost was {}: {:?}", cost, path);
        }
        None => println!("There was no path"),
    }
}
