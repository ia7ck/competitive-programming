use std::collections::BinaryHeap;

use proconio::input;

fn pow(a: u64, x: u64, m: u64) -> u64 {
    if x == 0 {
        1
    } else if x == 1 {
        a % m
    } else if x % 2 == 0 {
        pow(a * a % m, x / 2, m)
    } else {
        a * pow(a, x - 1, m) % m
    }
}

fn main() {
    input! {
        n: usize,
        m: u64,
        a: [u64; n],
    };

    let f = |x: u64, y: u64| -> u64 { (pow(x, y, m) + pow(y, x, m)) % m };

    let mut g = vec![vec![]; n];
    for i in 0..n {
        for j in (i + 1)..n {
            let w = f(a[i], a[j]);
            g[i].push((j, w));
            g[j].push((i, w));
        }
    }
    let mut seen = vec![false; n];
    let mut heap = BinaryHeap::new();
    seen[0] = true;
    for &(v, w) in &g[0] {
        heap.push((w, v));
    }
    let mut ans = 0;
    while let Some((w, u)) = heap.pop() {
        if seen[u] {
            continue;
        }
        seen[u] = true;
        ans += w;
        for &(v, ww) in &g[u] {
            if seen[v] {
                continue;
            }
            heap.push((ww, v));
        }
    }
    println!("{}", ans);
}
