use proconio::{input, marker::Usize1};
use dijkstra::{dijkstra, ConstEdge};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u64; n],
        edges: [(Usize1, Usize1, u64); m],
    };

    let edges = {
        let mut es = Vec::new();
        for (u, v, b) in edges {
            es.push(ConstEdge::new(u, v, b + a[v]));
            es.push(ConstEdge::new(v, u, b + a[u]));
        }
        es
    };

    let (d, _) = dijkstra(n, &edges, 0);
    for i in 1..n {
        let d = d[i].unwrap();
        let ans = a[0] + d;
        print!("{}", ans);
        if i + 1 < n {
            print!(" ");
        } else {
            print!("\n");
        }
    }
}
