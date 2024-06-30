use std::cmp::Reverse;

use proconio::input;
use segment_tree::SegmentTree;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };

    if n <= 2 {
        println!("{}", n);
        return;
    }

    let mut ord = (0..n).collect::<Vec<usize>>();
    ord.sort_by_key(|&i| (a[i], Reverse(i)));
    let mut dp_end = SegmentTree::new(n, 0, |x, y| *x.max(y));
    for &i in &ord {
        let x = 1 + dp_end.fold(0..i);
        dp_end.update(i, x);
    }

    ord.sort_by_key(|&i| (Reverse(a[i]), i));
    let mut dp_start = SegmentTree::new(n, 0, |x, y| *x.max(y));
    for &i in &ord {
        let x = 1 + dp_start.fold(i..n);
        dp_start.update(i, x);
    }

    let mut first_small = a.clone();
    first_small[0] = 0;
    let mut last_large = a.clone();
    last_large[n - 1] = 1_000_000_000 + 1;
    let mut ans = lis(n, &first_small).max(lis(n, &last_large));
    for i in 1..(n - 1) {
        if a[i - 1] + 1 < a[i + 1] {
            ans = ans.max(dp_end.get(i - 1) + 1 + dp_start.get(i + 1));
        }
    }
    println!("{}", ans);
}

fn lis(n: usize, a: &Vec<u32>) -> usize {
    let mut ord = (0..n).collect::<Vec<usize>>();
    ord.sort_by_key(|&i| (a[i], Reverse(i)));
    let mut dp_end = SegmentTree::new(n, 0, |x, y| *x.max(y));
    for &i in &ord {
        let x = 1 + dp_end.fold(0..i);
        dp_end.update(i, x);
    }
    dp_end.fold(0..n)
}
