use divisors::Divisors;
use mod_int::ModInt998244353;
use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let p: u64 = rd.get();

    let divisors = (p - 1).divisors();
    let mut dp = vec![0; divisors.len()];
    for (i, &d) in divisors.iter().enumerate().rev() {
        dp[i] = (p - 1) / d;
        for j in (i + 1)..divisors.len() {
            if divisors[j] % d == 0 {
                dp[i] -= dp[j];
            }
        }
    }
    {
        let sum = dp.iter().copied().sum::<u64>();
        assert_eq!(sum, p - 1);
    }
    type Mint = ModInt998244353;
    let mut ans = Mint::new(1); // (0, 0)
    for (i, &d) in divisors.iter().enumerate() {
        let a = Mint::new((p - 1) / d) * Mint::new(dp[i]);
        ans = ans + a;
    }
    println!("{}", ans.val());
}
