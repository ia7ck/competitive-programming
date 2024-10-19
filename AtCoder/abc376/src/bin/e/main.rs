use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            k: usize,
            a: [u64; n],
            b: [u64; n],
        };

        solve(n, k, a, b);
    }
}

fn solve(n: usize, k: usize, a: Vec<u64>, b: Vec<u64>) {
    let mut ord = (0..n).collect::<Vec<_>>();
    ord.sort_by_key(|&i| a[i]);

    let mut b_sum = 0;
    let mut heap = BinaryHeap::new();
    for &i in &ord[..k] {
        b_sum += b[i];
        heap.push(b[i]);
    }

    let mut ans = a[ord[k - 1]] * b_sum;
    for &i in &ord[k..n] {
        b_sum += b[i];
        heap.push(b[i]);
        b_sum -= heap.pop().unwrap();
        ans = ans.min(a[i] * b_sum);
    }

    println!("{}", ans);
}
