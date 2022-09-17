use proconio::input;
use proconio::marker::Usize1;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        q: usize,
        mut p: [Usize1; n - 1],
    };

    p.insert(0, std::usize::MAX);
    let p = p;

    let mut g = HashMap::new();
    for v in 0..n {
        g.insert(v, Vec::new());
    }
    for v in 1..n {
        g.get_mut(&p[v]).unwrap().push(v);
    }

    for _ in 0..q {
        input! {
            m: usize,
            v: [Usize1; m],
        };

        let v_set = v.iter().copied().collect::<HashSet<_>>();
        let mut roots = v_set.clone();
        let mut g_sub = HashMap::new();
        for &v in &v {
            g_sub.insert(v, Vec::new());
        }
        for &v in &v {
            if v == 0 {
                //
            } else {
                let p = p[v];
                if v_set.contains(&p) {
                    roots.remove(&v);
                    g_sub.get_mut(&p).unwrap().push(v);
                }
            }
        }
        let mut ans = 0;
        for r in roots {
            ans += 1;
            let mut que = VecDeque::new();
            que.push_back(r);
            while let Some(v) = que.pop_front() {
                assert!(g[&v].len() >= g_sub[&v].len());
                ans += g[&v].len() - g_sub[&v].len();
                for &child in &g_sub[&v] {
                    que.push_back(child);
                }
            }
        }
        println!("{}", ans);
    }
}
