use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let ws: Vec<(u64, u64)> = (0..n)
        .map(|_| {
            let w: u64 = rd.get();
            let s: u64 = rd.get();
            (w, s)
        })
        .collect();

    let mut ws = ws;
    ws.sort_by_key(|(w, s)| w + s);
    let inf = std::u64::MAX / 2;
    let mut dp = vec![vec![inf; n + 1]; n + 1];
    dp[0][0] = 0;
    for (i, &(w, s)) in ws.iter().enumerate() {
        for j in 0..=n {
            dp[i + 1][j] = dp[i + 1][j].min(dp[i][j]);
            if j >= 1 && s >= dp[i][j - 1] {
                dp[i + 1][j] = dp[i + 1][j].min(dp[i][j - 1] + w);
            }
        }
    }
    let ans = dp[n].iter().rposition(|&x| x != inf).unwrap();
    println!("{}", ans);
}
