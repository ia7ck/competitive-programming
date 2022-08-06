use proconio::input;

fn main() {
    input! {
        n: usize,
        l: i64,
        r: i64,
        a: [i64; n],
    };

    let mut dp = vec![vec![0; 2]; n];
    dp[0][0] = a[0];
    dp[0][1] = l;
    for i in 1..n {
        dp[i][0] = dp[i - 1][0].min(dp[i - 1][1]) + a[i];
        dp[i][1] = dp[i - 1][1] + l;
    }

    let mut ans = dp[n - 1][0].min(dp[n - 1][1]);
    let mut pd = vec![0; 2];
    for i in (0..n).rev() {
        pd = vec![pd[0].min(pd[1]) + a[i], pd[1] + r];
        let back = pd[0].min(pd[1]);
        if i == 0 {
            ans = ans.min(back);
        } else {
            let front = dp[i - 1][0].min(dp[i - 1][1]);
            ans = ans.min(front + back);
        }
    }
    println!("{}", ans);
}
