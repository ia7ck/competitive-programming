use proconio::input;
use rustc_hash::FxHashSet;

const BASE: u64 = 123456789;
const MOD1: u64 = 999999733;
const MOD2: u64 = 1000000007;

fn main() {
    input! {
        s: String,
        n: usize,
        tt: [String; n],
    }

    const L: usize = 500;
    let mut small = vec![FxHashSet::default(); L + 1];
    let mut large = Vec::new();
    for t in &tt {
        let mut h1 = 0;
        let mut h2 = 0;
        for c in t.chars() {
            h1 = (h1 + u64::from(c)) * BASE % MOD1;
            h2 = (h2 + u64::from(c)) * BASE % MOD2;
        }
        if t.len() <= L {
            small[t.len()].insert((h1, h2));
        } else if t.len() <= s.len() {
            large.push((h1, h2, t.len()));
        }
    }
    large.sort_by_key(|&(_, _, l)| l);

    let mut ans = 0;
    let mut pows1 = vec![1; s.len() + 1];
    let mut pows2 = vec![1; s.len() + 1];
    let mut hs1 = vec![0; s.len() + 1];
    let mut hs2 = vec![0; s.len() + 1];
    for (i, c) in s.chars().enumerate() {
        hs1[i + 1] = (hs1[i] + u64::from(c)) * BASE % MOD1;
        hs2[i + 1] = (hs2[i] + u64::from(c)) * BASE % MOD2;
        pows1[i + 1] = pows1[i] * BASE % MOD1;
        pows2[i + 1] = pows2[i] * BASE % MOD2;

        let mut found = false;
        // small
        for j in i.saturating_sub(L - 1)..=i {
            let l = i - j + 1;
            let h1 = (MOD1 + hs1[i + 1] - hs1[j] * pows1[l] % MOD1) % MOD1;
            let h2 = (MOD2 + hs2[i + 1] - hs2[j] * pows2[l] % MOD2) % MOD2;
            if small[l].contains(&(h1, h2)) {
                found = true;
                break;
            }
        }
        // large
        if !found {
            for &(h1, h2, l) in &large {
                if i + 1 >= l {
                    let j = i + 1 - l;
                    let hh1 = (MOD1 + hs1[i + 1] - hs1[j] * pows1[l] % MOD1) % MOD1;
                    let hh2 = (MOD2 + hs2[i + 1] - hs2[j] * pows2[l] % MOD2) % MOD2;
                    if (h1, h2) == (hh1, hh2) {
                        found = true;
                        break;
                    }
                } else {
                    // 定数倍高速化
                    break;
                }
            }
        }
        if found {
            ans += 1;
            hs1[i + 1] = (hs1[i] + u64::from('*')) * BASE % MOD1;
            hs2[i + 1] = (hs2[i] + u64::from('*')) * BASE % MOD2
        }
    }

    println!("{}", ans);
}
