use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{
    input,
    marker::{Bytes, Usize1},
};

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        s: [Bytes; n],
        q: usize,
        uv: [(Usize1, Usize1); q],
    };

    let mut g = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == b'Y' {
                g[i].push(j);
            }
        }
    }

    const INF: usize = std::usize::MAX;
    let mut ans = vec![vec![(INF, 0); n]; n];
    for s in 0..n {
        let ans = &mut ans[s];
        let mut heap = BinaryHeap::new();
        ans[s] = (0, a[s]);
        heap.push((Reverse(0), a[s], s));
        while let Some((Reverse(d), c, i)) = heap.pop() {
            let (dd, cc) = ans[i];
            if dd < d || (dd == d && cc > c) {
                continue;
            }
            for &j in &g[i] {
                let (dd, cc) = ans[j];
                if d + 1 < dd || (d + 1 == dd && c + a[j] > cc) {
                    ans[j] = (d + 1, c + a[j]);
                    heap.push((Reverse(d + 1), c + a[j], j));
                }
            }
        }
    }

    for (u, v) in uv {
        let (d, c) = ans[u][v];
        if d == INF {
            println!("Impossible");
        } else {
            println!("{} {}", d, c);
        }
    }
}
