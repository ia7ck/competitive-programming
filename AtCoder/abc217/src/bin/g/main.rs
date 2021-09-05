use factorials::factorials;
use mod_int::ModInt998244353;
use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let m: usize = rd.get();

    let (fac, inv_fac) = factorials(n + 1, ModInt998244353::p() as u64);
    let binom = |a: usize, b: usize| {
        if a < b {
            return 0.into();
        }
        ModInt998244353::from(fac[a]) * inv_fac[b] * inv_fac[a - b]
    };

    // a[i] := n 人を mod m が被らないように i グループに分ける通り数
    let mut a = vec![ModInt998244353::new(0); n + 1];
    for i in 1..=n {
        a[i] = binom(i, n / m) * fac[n / m];
        for j in 1..m {
            // k = 1, 2, ..., n のうち k % m = j を満たす k の個数 ... (n-j)/m+1
            a[i] *= binom(i, (n - j) / m + 1) * fac[(n - j) / m + 1];
        }
    }

    for k in 1..=n {
        let mut ans = a[k];
        for t in 1..=k {
            // 空のグループが t 個以上
            let x = binom(k, t) * a[k - t];
            if t % 2 == 1 {
                ans -= x;
            } else {
                ans += x;
            }
        }
        println!("{}", (ans / fac[k]).val());
    }
}
