use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        events: [(u64, u64, u64); m],
    };

    let mut row = BinaryHeap::new();
    let mut wait = BinaryHeap::new();
    for i in 0..n {
        row.push(Reverse(i));
    }
    let mut ans = vec![0; n];
    for (t, w, s) in events {
        loop {
            let Some(&(Reverse(back), i)) = wait.peek() else {
                break;
            };
            if t < back {
                break;
            }
            wait.pop(); // (back ,i)
            row.push(Reverse(i));
        }
        if let Some(Reverse(i)) = row.pop() {
            ans[i] += w;
            wait.push((Reverse(t + s), i));
        }
    }
    for ans in ans {
        println!("{}", ans);
    }
}
