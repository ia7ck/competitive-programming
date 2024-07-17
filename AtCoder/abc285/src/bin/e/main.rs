use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    };

    let mut b = vec![0; n + 1];
    for i in 1..=n {
        for j in 0..i {
            b[i] += a[j.min(i - j - 1)];
        }
    }
    let mut dp = vec![0; n + 2];
    for i in 2..=(n + 1) {
        for j in 1..i {
            // j + 1, j + 2, ..., i - 1 の
            // i - j - 1 日間を平日にする
            dp[i] = dp[i].max(dp[j] + b[i - j - 1]);
        }
    }
    println!("{}", dp[n + 1]);
}
