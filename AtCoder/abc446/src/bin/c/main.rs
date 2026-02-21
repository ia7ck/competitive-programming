use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            d: usize,
            a: [usize; n],
            b: [usize; n],
        };

        solve(n, d, a, b);
    }
}

fn solve(n: usize, d: usize, a: Vec<usize>, b: Vec<usize>) {
    let mut que = VecDeque::new();
    for i in 0..n {
        let (a, b) = (a[i], b[i]);
        for _ in 0..a {
            que.push_back(i);
        }
        for _ in 0..b {
            que.pop_front().unwrap();
        }
        while let Some(&j) = que.front() {
            if i - j >= d {
                // j
                que.pop_front();
            } else {
                break;
            }
        }
    }

    println!("{}", que.len());
}
