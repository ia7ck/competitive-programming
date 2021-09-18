use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let x: usize = rd.get();
    let y: usize = rd.get();
    let ab: Vec<(usize, usize)> = (0..n)
        .map(|_| {
            let a: usize = rd.get();
            let b: usize = rd.get();
            (a, b)
        })
        .collect();

    let inf = std::usize::MAX / 2;
    let mut dp = vec![vec![inf; y + 1]; x + 1];
    dp[0][0] = 0;
    for (a, b) in ab {
        let mut nxt = dp.clone();
        for i in 0..=x {
            for j in 0..=y {
                nxt[x.min(i + a)][y.min(j + b)] = nxt[x.min(i + a)][y.min(j + b)].min(dp[i][j] + 1);
            }
        }
        dp = nxt;
    }
    let ans = dp[x][y];
    if ans == inf {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
