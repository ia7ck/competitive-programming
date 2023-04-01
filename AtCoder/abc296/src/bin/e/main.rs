use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    };

    let mut in_deg = vec![0; n];
    for v in 0..n {
        in_deg[a[v]] += 1;
    }
    let mut seen = vec![false; n];
    let mut win = vec![false; n];
    for s in 0..n {
        if in_deg[s] > 0 {
            continue;
        }
        let mut v = s;
        let mut route = HashSet::new();
        while seen[v] == false && route.contains(&v) == false {
            seen[v] = true;
            route.insert(v);
            v = a[v];
        }
        if route.contains(&v) == false {
            continue;
        }
        assert!(seen[v]);
        route.clear();
        while route.contains(&v) == false {
            win[v] = true;
            route.insert(v);
            v = a[v];
        }
    }

    for v in 0..n {
        if seen[v] == false {
            win[v] = true;
        }
    }

    let ans = win.iter().filter(|&&c| c).count();
    println!("{}", ans);
}
