use dijkstra::{dijkstra, ConstEdge};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u64; n],
        lrc: [(Usize1, Usize1, u64); m],
    };

    let mut edges = Vec::new();
    for i in 0..n {
        edges.push(ConstEdge::new(i, i + 1, a[i]));
    }
    for (l, r, c) in lrc {
        edges.push(ConstEdge::new(l, r + 1, c));
    }
    for i in 0..n {
        edges.push(ConstEdge::new(i + 1, i, 0));
    }

    let (d, _) = dijkstra(n + 1, &edges, 0);
    let y = d[n].unwrap();

    let x = a.iter().sum::<u64>();
    println!("{}", x - y);
}
