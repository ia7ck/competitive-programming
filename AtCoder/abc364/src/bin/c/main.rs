use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u64,
        y: u64,
        a: [u64; n],
        b: [u64; n],
    };

    let mut ord = (0..n).collect::<Vec<_>>();
    ord.sort_by_key(|&i| Reverse(a[i]));
    let mut sum = 0;
    let mut ans_a = 0;
    while ans_a < n {
        sum += a[ord[ans_a]];
        ans_a += 1;
        if sum > x {
            break;
        }
    }

    ord.sort_by_key(|&i| Reverse(b[i]));
    let mut sum = 0;
    let mut ans_b = 0;
    while ans_b < n {
        sum += b[ord[ans_b]];
        ans_b += 1;
        if sum > y {
            break;
        }
    }

    let ans = ans_a.min(ans_b);
    println!("{}", ans);
}
