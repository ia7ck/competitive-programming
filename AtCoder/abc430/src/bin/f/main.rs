use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            s: Chars,
        };

        solve(n, s);
    }
}

fn solve(n: usize, s: Vec<char>) {
    let mut low = vec![0; n];
    let mut high = vec![0; n];

    let mut g = vec![vec![]; n];
    let mut rg = vec![vec![]; n];
    for i in 0..(n - 1) {
        if s[i] == 'L' {
            g[i + 1].push(i);
            rg[i].push(i + 1);
        } else {
            g[i].push(i + 1);
            rg[i + 1].push(i);
        }
    }
    let mut que = VecDeque::new();
    for i in 0..n {
        if rg[i].is_empty() {
            low[i] = 0;
            que.push_back(i);
        }
    }
    let mut count = vec![0; n];
    while let Some(i) = que.pop_front() {
        for &j in &g[i] {
            count[j] += 1;
            low[j] += low[i] + 1;
            if count[j] == rg[j].len() {
                que.push_back(j);
            }
        }
    }
    let mut que = VecDeque::new();
    for i in 0..n {
        if g[i].is_empty() {
            high[i] = 0;
            que.push_back(i);
        }
    }
    let mut count = vec![0; n];
    while let Some(i) = que.pop_front() {
        for &j in &rg[i] {
            count[j] += 1;
            high[j] += high[i] + 1;
            if count[j] == g[j].len() {
                que.push_back(j);
            }
        }
    }
    for i in 0..n {
        high[i] = (n - 1) - high[i];
    }

    let mut ans = vec![0_isize; n];
    for i in 0..n {
        assert!(low[i] <= high[i]);
        ans[low[i]] += 1;
        if high[i] + 1 < n {
            ans[high[i] + 1] -= 1;
        }
    }
    for i in 1..n {
        ans[i] += ans[i - 1];
    }
    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
