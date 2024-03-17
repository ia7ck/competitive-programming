use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        c: i64,
        a: [i64; n],
    };

    let mut cum_sum_a = vec![0; n + 1];
    for i in 0..n {
        cum_sum_a[i + 1] = cum_sum_a[i] + a[i];
    }
    let mut cum_sum_a_x_c = vec![0; n + 1];
    for i in 0..n {
        cum_sum_a_x_c[i + 1] = cum_sum_a_x_c[i] + a[i] * c;
    }

    let mut heap = BinaryHeap::new();
    heap.push(Reverse(0));
    let mut ans = a.iter().sum::<i64>();
    for r in 0..n {
        let tail = cum_sum_a[n] - cum_sum_a[r + 1];
        let y = cum_sum_a_x_c[r + 1];
        let Reverse(x) = heap.peek().copied().unwrap();
        ans = ans.max((y - x) + tail);
        heap.push(Reverse(cum_sum_a_x_c[r + 1] - cum_sum_a[r + 1]));
    }

    println!("{}", ans);
}
