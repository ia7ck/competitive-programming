use mod_int::ModInt998244353;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    if 2_usize.saturating_pow((n - 1) as u32) > m {
        println!("0");
        return;
    }

    assert!(n <= 60);

    // // prev_power_of_two
    // let p = if m.is_power_of_two() {
    //     m
    // } else {
    //     m.next_power_of_two() / 2
    // };

    let mut k = 0;
    for i in 0..60 {
        if m >> i & 1 == 1 {
            k = i;
        }
    }
    dbg!(k);
    assert!(2_usize.pow(k as u32) <= m);
    assert!(m < 2_usize.pow(k as u32 + 1));

    type Mint = ModInt998244353;

    let mut dp = vec![Mint::new(0); k + 1];
    for j in 0..k {
        // msb(*) = j
        dp[j] = Mint::new(2).pow(j);
    }
    dp[k] = Mint::from(m) - Mint::new(2).pow(k) + 1;

    for _ in 1..n {
        let mut nxt = vec![Mint::new(0); k + 1];
        for j in 0..k {
            for t in (j + 1)..k {
                nxt[t] += dp[j] * 2_usize.pow(t as u32);
            }
            nxt[k] += dp[j] * (m - 2_usize.pow(k as u32) + 1);
        }
        dp = nxt;
    }

    let mut ans = Mint::new(0);
    for dp in dp {
        ans += dp;
    }
    println!("{}", ans.val());
}
