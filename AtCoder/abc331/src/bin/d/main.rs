use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        q: usize,
        p: [Chars; n],
        queries: [(usize, usize, usize, usize); q],
    };

    let mut cum_sum = vec![vec![0_u64; n + 1]; n + 1];
    for i in 0..n {
        for j in 0..n {
            cum_sum[i + 1][j + 1] =
                cum_sum[i + 1][j] + cum_sum[i][j + 1] - cum_sum[i][j] + u64::from(p[i][j] == 'B');
        }
    }

    let count = |h: usize, w: usize| {
        let h_q = (h / n) as u64;
        let w_q = (w / n) as u64;
        cum_sum[n][n] * h_q * w_q
            + cum_sum[h % n][n] * w_q
            + cum_sum[n][w % n] * h_q
            + cum_sum[h % n][w % n]
    };

    for (a, b, c, d) in queries {
        // let ans = count(c + 1, d + 1) - count(a, d + 1) - count(c + 1, b) + count(a, b);
        let ans = count(c + 1, d + 1)
            .wrapping_sub(count(a, d + 1))
            .wrapping_sub(count(c + 1, b))
            .wrapping_add(count(a, b));
        println!("{}", ans);
    }
}
