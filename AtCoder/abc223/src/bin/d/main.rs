use input_i_scanner::{scan_with, InputIScanner};
use join::Join;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let (n, m) = scan_with!(_i_i, (usize, usize));
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        let (a, b) = scan_with!(_i_i, (usize, usize));
        g[a - 1].push(b - 1);
    }

    let mut deg = vec![0; n];
    for u in 0..n {
        for &v in &g[u] {
            deg[v] += 1;
        }
    }
    let mut heap = BinaryHeap::new();
    let mut seen = vec![false; n];
    let mut perm = Vec::new();
    for v in 0..n {
        if deg[v] == 0 {
            heap.push(Reverse(v));
        }
    }
    while let Some(Reverse(u)) = heap.pop() {
        if seen[u] {
            continue;
        }
        seen[u] = true;
        perm.push(u);
        for &v in &g[u] {
            if seen[v] {
                continue;
            }
            deg[v] -= 1;
            if deg[v] == 0 {
                heap.push(Reverse(v));
            }
        }
    }
    if perm.len() == n {
        println!("{}", perm.iter().map(|x| x + 1).join(" "));
    } else {
        println!("-1");
    }
}
