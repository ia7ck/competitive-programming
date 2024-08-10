use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut ab: [(u64, u64); n],
    };

    // ai * (aj + bj) + bi <> aj * (ai + bi) + bj
    // ai * bj + bi <> aj * bi + bj
    // ai / bi + 1 / bj <> aj / bj + 1 / bi
    // (ai - 1) / bi <> (aj - 1) / bj

    ab.sort_by(|&(a, b), &(aa, bb)| ((a - 1) * bb).cmp(&((aa - 1) * b)));
    
    let mut dp = vec![0; k + 1];
    dp[0] = 1;
    for (a, b) in ab {
        let mut new_dp = dp.clone();
        for i in 0..k {
            if dp[i] >= 1 {
                new_dp[i + 1] = new_dp[i + 1].max(a * dp[i] + b);
            }
        }
        dp = new_dp;
    }

    println!("{}", dp[k]);
}
