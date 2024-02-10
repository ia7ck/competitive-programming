use proconio::{input, marker::Usize1};
use dijkstra::{dijkstra, ConstEdge};

fn main() {
    input! {
        n: usize,
        abx: [(u64, u64, Usize1); n - 1],
    };

    let mut edges = Vec::new();
    for i in 0..(n - 1) {
        let (a, b, x) = abx[i];
        edges.push(ConstEdge::new(i, i + 1, a));
        edges.push(ConstEdge::new(i, x, b));
    }

    let (dist, _) = dijkstra(n, &edges, 0);
    let ans = dist[n - 1].unwrap();
    println!("{}", ans);
}
