use proconio::input;

fn main() {
    input! {
        n: usize,
        h: usize,
        mut ab: [(usize, usize); n],
    };

    ab.sort_by(|(a, b), (aa, bb)| {
        // a / b <=> aa / bb
        (a * bb).cmp(&(aa * b))
    });
    ab.reverse();

    let mut dp = vec![0; h + 1];
    for (a, b) in ab {
        for i in 1..=h {
            let j = i.saturating_sub(b);
            dp[j] = dp[j].max(dp[i] + a * i);
        }
    }

    let ans = dp.iter().copied().max().unwrap();
    println!("{}", ans);
}
