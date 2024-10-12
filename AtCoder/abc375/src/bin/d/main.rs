use proconio::{input, marker::Bytes};

fn main() {
    input! {
        s: Bytes,
    };

    let mut cum_sum = vec![vec![0; s.len() + 1]; 26];
    for c in b'A'..=b'Z' {
        for i in 0..s.len() {
            cum_sum[usize::from(c - b'A')][i + 1] = cum_sum[usize::from(c - b'A')][i];
            if s[i] == c {
                cum_sum[usize::from(c - b'A')][i + 1] += 1;
            }
        }
    }

    let mut ans = 0_usize;
    for j in 1..(s.len() - 1) {
        for c in 0..26 {
            let left = cum_sum[c][j];
            let right = cum_sum[c][s.len()] - cum_sum[c][j + 1];
            ans += left * right;
        }
    }

    println!("{}", ans);
}
