use dijkstra::{dijkstra, ConstEdge};
use join::Join;
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize, u64); m],
    };

    let mut d_edges = Vec::new();
    let mut edge_id = HashMap::new();
    for (i, &(a, b, c)) in edges.iter().enumerate() {
        d_edges.push(ConstEdge::new(a - 1, b - 1, c));
        d_edges.push(ConstEdge::new(b - 1, a - 1, c));
        edge_id.insert((a - 1, b - 1), i);
        edge_id.insert((b - 1, a - 1), i);
    }

    let (_d, prev) = dijkstra(n, d_edges.iter().copied(), 0);
    let mut ans = Vec::new();
    for i in 1..n {
        let p = prev[i].unwrap();
        ans.push(edge_id[&(p, i)]);
    }
    println!("{}", ans.iter().map(|&i| i + 1).join(" "));
}
