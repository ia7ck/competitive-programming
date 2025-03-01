use dijkstra::{dijkstra, ConstEdge};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        x: u64,
        edges: [(Usize1, Usize1); m],
    };

    let id = |v: usize, rev: bool| {
        if rev {
            n + v
        } else {
            v
        }
    };

    let edges = {
        let mut new_edges = Vec::new();
        for &(u, v) in &edges {
            new_edges.push(ConstEdge::new(id(u, false), id(v, false), 1));
            new_edges.push(ConstEdge::new(id(v, true), id(u, true), 1));
        }
        for v in 0..n {
            new_edges.push(ConstEdge::new(id(v, false), id(v, true), x));
            new_edges.push(ConstEdge::new(id(v, true), id(v, false), x));
        }
        new_edges
    };

    let (dist, _) = dijkstra(n * 2, &edges, id(0, false));
    let mut ans = u64::MAX;
    for rev in [false, true] {
        if let Some(d) = dist[id(n - 1, rev)] {
            ans = ans.min(d);
        }
    }
    assert_ne!(ans, u64::MAX);
    println!("{}", ans);
}
