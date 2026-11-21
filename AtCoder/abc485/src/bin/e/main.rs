use std::collections::{HashMap, HashSet};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1, usize); m],
    };

    let mut g = vec![vec![]; n];
    let mut rg = vec![vec![]; n];
    for &(u, v, c) in &edges {
        rg[v].push((u, c));
        g[u].push((v, c, rg[v].len() - 1));
    }

    let mut d = vec![vec![]; n];
    for i in 0..n {
        d[i] = vec![usize::MAX; rg[i].len()];
    }

    let mut que = Vec::<(usize, usize)>::new();
    for &(v, _, k) in &g[0] {
        if d[v][k] == usize::MAX {
            d[v][k] = 1;
            que.push((v, k));
        }
    }
    for t in 2.. {
        let mut new_q = Vec::new();

        let mut colors_by_x = HashMap::<usize, HashSet<usize>>::new();
        for (x, k) in que {
            colors_by_x.entry(x).or_default().insert(rg[x][k].1);
        }
        for (x, cs) in colors_by_x {
            for &(y, c, k) in &g[x] {
                if cs.len() > 1 || !cs.contains(&c) {
                    if d[y][k] == usize::MAX {
                        d[y][k] = t;
                        new_q.push((y, k));
                    }
                }
            }
        }

        que = new_q;

        if que.is_empty() {
            break;
        }
    }

    let mut ans = usize::MAX;
    for k in 0..d[n - 1].len() {
        ans = ans.min(d[n - 1][k]);
    }
    if ans == usize::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
