use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let t: Vec<usize> = rd.get_vec(n);

    macro_rules! chmin {
        ($a: expr, $b: expr) => {
            $a = std::cmp::min($a, $b);
        };
    }

    let inf = std::usize::MAX / 2;
    let mut dp = vec![inf; 1_000_00 + 1];
    dp[0] = 0;
    for &t in &t {
        let mut nxt = vec![inf; dp.len()];
        for s in 0..dp.len() {
            if s + t < dp.len() {
                chmin!(nxt[s + t], dp[s]);
            }
            chmin!(nxt[s], dp[s] + t);
        }
        dp = nxt;
    }
    let mut ans = inf;
    for i in 0..dp.len() {
        chmin!(ans, i.max(dp[i]));
    }
    println!("{}", ans);
}
