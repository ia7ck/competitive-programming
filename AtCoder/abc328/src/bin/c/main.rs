use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        lr: [(usize, usize); q],
    };

    let mut cum_sum = vec![0; n + 1];
    for i in 0..n {
        cum_sum[i + 1] = cum_sum[i];
        if i + 1 < n && s[i] == s[i + 1] {
            cum_sum[i + 1] += 1;
        }
    }

    for (l, r) in lr {
        let ans = cum_sum[r - 1] - cum_sum[l - 1];
        println!("{}", ans);
    }
}
