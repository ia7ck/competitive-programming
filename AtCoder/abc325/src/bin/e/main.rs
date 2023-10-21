use dijkstra::{dijkstra, ConstEdge};
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: u64,
        b: u64,
        c: u64,
        d: [[u64; n]; n],
    };

    let id = |v: usize, used: bool| {
        if used == false {
            v
        } else {
            v + n
        }
    };

    let mut edges = Vec::new();
    for i in 0..n {
        let i0 = id(i, false);
        let i1 = id(i, true);
        for j in 0..n {
            let j0 = id(j, false);
            let j1 = id(j, true);
            edges.push(ConstEdge::new(i0, j0, d[i][j] * a));
            edges.push(ConstEdge::new(i0, j1, d[i][j] * b + c));
            edges.push(ConstEdge::new(i1, j1, d[i][j] * b + c));
        }
    }

    let start = id(0, false);
    let (f, _) = dijkstra(n * 2, &edges, start);
    let ans = f[id(n - 1, false)].min(f[id(n - 1, true)]).unwrap();
    println!("{}", ans);
}
