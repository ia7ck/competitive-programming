use std::collections::{BinaryHeap, HashSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [u64; n],
        mut b: [u64; n],
        mut c: [u64; n],
    };
    a.sort_unstable();
    a.reverse();
    b.sort_unstable();
    b.reverse();
    c.sort_unstable();
    c.reverse();

    let mut heap = BinaryHeap::new();
    let mut push = HashSet::new();
    heap.push((a[0] * b[0] + b[0] * c[0] + c[0] * a[0], 0, 0, 0));
    push.insert((0, 0, 0));
    let mut pop = 0;
    while let Some((v, i, j, k)) = heap.pop() {
        pop += 1;
        if pop == m {
            println!("{}", v);
            return;
        }
        let mut current = vec![(i, j, k)];
        while let Some(&(x, i, j, k)) = heap.peek() {
            if x < v {
                break;
            }
            heap.pop(); // (x, i, j, k)
            pop += 1;
            if pop == m {
                println!("{}", x); // = v
                return;
            }
            current.push((i, j, k));
        }
        for (i, j, k) in current {
            for di in 0..=1 {
                for dj in 0..=1 {
                    for dk in 0..=1 {
                        if (di, dj, dk) == (0, 0, 0) {
                            continue;
                        }
                        let (ni, nj, nk) = (i + di, j + dj, k + dk);
                        if ni >= n || nj >= n || nk >= n {
                            continue;
                        }
                        if push.contains(&(ni, nj, nk)) {
                            continue;
                        }
                        heap.push((a[ni] * b[nj] + b[nj] * c[nk] + c[nk] * a[ni], ni, nj, nk));
                        push.insert((ni, nj, nk));
                    }
                }
            }
        }
    }

    unreachable!()
}
