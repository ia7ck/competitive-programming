use std::collections::{BinaryHeap, HashMap};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            s: Chars,
        };

        solve(s);
    }
}

fn solve(s: Vec<char>) {
    let mut counter = HashMap::new();
    for c in s {
        counter.entry(c).and_modify(|e| *e += 1).or_insert(0);
    }
    let mut heap = BinaryHeap::new();
    for (k, v) in counter {
        heap.push((v, k));
    }
    let mut ans = Vec::new();
    while let Some((k0, c0)) = heap.pop() {
        // eprintln!("{:?}, heap = {heap:?}, ans = {ans:?}", (k0, c0));
        if ans.last() == Some(&c0) {
            let Some((k1, c1)) = heap.pop() else {
                println!("No");
                return;
            };
            ans.push(c1);
            if k1 >= 1 {
                heap.push((k1 - 1, c1));
            }
            heap.push((k0, c0));
        } else {
            ans.push(c0);
            if k0 >= 1 {
                heap.push((k0 - 1, c0));
            }
        }
    }
    println!("Yes");
    println!("{}", ans.iter().collect::<String>());
}

