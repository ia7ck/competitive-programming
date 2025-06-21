use std::collections::{BTreeMap, HashMap};

use mod_int::ModInt998244353;
use proconio::input;

type Mint = ModInt998244353;

fn main() {
    input! {
        n: usize,
        a: [[u32; 6]; n],
    };

    let mut dice = BTreeMap::<u32, Vec<usize>>::new();
    for i in 0..n {
        for &x in &a[i] {
            dice.entry(x).or_default().push(i);
        }
    }

    // q[k] := maxがk以下になる確率
    // サイコロ1の出目がk以下 × サイコロ2の出目がk以下 × ...
    let mut q = HashMap::<u32, Mint>::new();
    let mut seen = vec![0; n];
    let mut zero = n;
    // non-zero部分の総積
    let mut prod = Mint::new(1);
    for (&x, dice) in dice.iter() {
        let mut count = HashMap::<usize, usize>::new();
        for &i in dice {
            *count.entry(i).or_insert(0) += 1;
        }
        for (i, value) in count {
            if seen[i] == 0 {
                zero -= 1;
            } else {
                prod /= seen[i];
            }
            seen[i] += value;
            prod *= seen[i];
        }
        q.insert(
            x,
            if zero == 0 {
                prod / Mint::new(6).pow(n as u32)
            } else {
                Mint::new(0)
            },
        );
    }

    let mut ans = Mint::new(0);
    let mut y = None;
    for (x, _) in dice {
        let p = match y {
            None => q[&x],
            Some(y) => q[&x] - q[&y],
        };
        ans += Mint::from(x) * p;
        y = Some(x);
    }

    println!("{}", ans.val());
}
