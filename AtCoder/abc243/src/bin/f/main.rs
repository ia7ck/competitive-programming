use mod_int::ModInt998244353;
use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let (n, m, k) = scan!((usize, usize, usize));
    let w = scan!(u32; n);

    type Mint = ModInt998244353;

    let mut fact = vec![Mint::new(1)];
    for i in 1..=k {
        fact.push(fact[i - 1] * i);
    }

    let mut tot = Mint::new(0);
    for &w in &w {
        tot += w;
    }
    let p: Vec<Mint> = w.into_iter().map(|w| Mint::from(w) / tot).collect();

    let mut dp = vec![vec![Mint::new(0); k + 1]; m + 1];
    dp[0][0] = Mint::new(1);
    for i in 0..n {
        let mut nxt = vec![vec![Mint::new(0); k + 1]; m + 1];
        for a in 0..=m {
            for s in 0..=k {
                // choose i-th item, t times
                for t in 0..=k {
                    if s + t <= k {
                        if t == 0 {
                            nxt[a][s + t] += dp[a][s] / fact[t] * p[i].pow(t);
                        } else {
                            if a + 1 <= m {
                                nxt[a + 1][s + t] += dp[a][s] / fact[t] * p[i].pow(t);
                            }
                        }
                    }
                }
            }
        }
        dp = nxt;
    }
    let ans = dp[m][k] * fact[k];
    println!("{}", ans.val());
}
