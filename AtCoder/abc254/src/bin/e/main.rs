use proconio::input;
use std::collections::{HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
        q: usize,
        xk: [(usize, usize); q],
    };

    let mut g = vec![vec![]; n];
    for (a, b) in ab {
        g[a - 1].push(b - 1);
        g[b - 1].push(a - 1);
    }
    for (x, k) in xk {
        let mut seen = HashSet::new();
        let mut que = VecDeque::new();
        seen.insert(x - 1);
        que.push_back((x - 1, 0));
        while let Some((u, d)) = que.pop_front() {
            if d == k {
                continue;
            }
            for &v in &g[u] {
                if !seen.contains(&v) {
                    seen.insert(v);
                    que.push_back((v, d + 1));
                }
            }
        }
        let mut ans = 0;
        for u in seen {
            ans += u + 1;
        }
        println!("{}", ans);
    }
}
