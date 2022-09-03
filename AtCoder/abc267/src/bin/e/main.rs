use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u64; n],
        edges: [(Usize1, Usize1); m],
    };

    let mut g = vec![vec![]; n];
    for (u, v) in edges {
        g[u].push(v);
        g[v].push(u);
    }

    let mut cost = vec![0; n];
    for i in 0..n {
        for &j in &g[i] {
            cost[i] += a[j];
        }
    }
    if cost.iter().all(|&c| c == 0) {
        println!("0");
        return;
    }

    let mut ng = 0;
    let mut ok = a.iter().sum::<u64>() + 1;
    while ok - ng > 1 {
        let mid = (ok + ng) / 2;
        let mut cost = cost.clone();
        let mut seen = vec![false; n];
        let mut heap = VecDeque::new();
        for i in 0..n {
            if cost[i] <= mid {
                seen[i] = true;
                heap.push_back(i);
                cost[i] = 0; // ??
            }
        }
        let mut remove_count = 0;
        while let Some(i) = heap.pop_front() {
            remove_count += 1;
            for &j in &g[i] {
                if seen[j] {
                    continue;
                }
                assert!(cost[j] >= a[i]);
                cost[j] -= a[i];
                if cost[j] <= mid {
                    seen[j] = true;
                    heap.push_back(j);
                    cost[j] = 0;
                }
            }
        }
        if remove_count == n {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}
