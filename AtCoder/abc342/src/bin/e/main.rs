use std::collections::BinaryHeap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ldkcab: [(u64, u64, u64, u64, Usize1, Usize1); m],
    };

    let mut edges = Vec::new();
    for (l, d, k, c, a, b) in ldkcab {
        edges.push(Data { l, d, k, c, a, b });
    }

    let ans = solve(n, &edges, n - 1);
    for i in 0..n - 1 {
        if let Some(ans) = ans[i] {
            println!("{}", ans);
        } else {
            println!("Unreachable");
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Data {
    l: u64,
    d: u64,
    k: u64,
    c: u64,
    a: usize,
    b: usize,
}

fn solve(n: usize, edges: &Vec<Data>, s: usize) -> Vec<Option<u64>> {
    let mut graph = vec![vec![]; n];
    for &e in edges {
        graph[e.b].push(e);
    }

    let mut time = vec![None; n];
    let mut heap = BinaryHeap::new();
    time[s] = Some(u64::MAX / 3);
    heap.push((u64::MAX / 3, s));
    while let Some((t, v)) = heap.pop() {
        match time[v] {
            Some(tv) => {
                if tv > t {
                    continue;
                } else {
                    assert_eq!(tv, t);
                }
            }
            None => unreachable!(),
        }
        for e in &graph[v] {
            if t < e.c {
                continue;
            }
            // e.l + k * e.d <= t - e.c
            let k = (t - e.c).saturating_sub(e.l) / e.d;
            if e.l + k * e.d <= t - e.c {
                let next_t = e.l + k.min(e.k - 1) * e.d;
                match time[e.a] {
                    Some(ta) if ta >= next_t => {
                        continue;
                    }
                    _ => {
                        time[e.a] = Some(next_t);
                        heap.push((next_t, e.a));
                    }
                }
            }
        }
    }
    time
}
