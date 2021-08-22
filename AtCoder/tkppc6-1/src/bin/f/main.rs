use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let k: usize = rd.get();

    let mo: u64 = 998244353;
    macro_rules! sub {
        ($a: expr, $b: expr) => {
            $a = (mo + $a % mo - $b % mo) % mo;
        };
    }

    let mut pow2 = vec![0; n + 1];
    pow2[0] = 1;
    for i in 1..=n {
        pow2[i] = pow2[i - 1] * 2 % mo;
    }

    let mut dp = vec![0; n + 1];
    let mut ep = vec![0; n + 1];
    dp[1] = 1;
    ep[1] = 1;
    for i in 2..=n {
        dp[i] = ep[i - 1];
        if i >= k + 1 {
            sub!(dp[i], ep[i - k - 1] * pow2[k]);
        }
        ep[i] = ((ep[i - 1] * 2) % mo + dp[i]) % mo;
    }

    println!("{}", dp[n]);
}
