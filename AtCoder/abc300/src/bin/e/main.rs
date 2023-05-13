use std::collections::HashMap;

use mod_int::ModInt998244353;
use proconio::input;

// p(6) = 1/6 * p(6) + 1/6 * p(3) + 1/6 * p(2) + 1/6 * p(1)
// p(3) = 1/6 * p(3) + 1/6 * p(1)
// p(2) = 1/6 * p(2) + 1/6 * p(1)
//
// 5/6 * p(6) = 1/6 * 1/5 + 1/6 * 1/5 + 1/6
// p(6) = (1/5 + 1/5 + 1) / 5 = 7/25

type Mint = ModInt998244353;

fn solve(n: u64, memo: &mut HashMap<u64, Mint>) -> Mint {
    if n == 1 {
        return Mint::new(1);
    }

    if let Some(&p) = memo.get(&n) {
        return p;
    }

    let mut p = Mint::new(0);
    for i in 2..=6 {
        if n % i == 0 {
            p += solve(n / i, memo) / Mint::new(5);
        }
    }
    memo.insert(n, p);
    p
}

fn main() {
    input! {
        n: u64,
    };

    let mut memo = HashMap::new();
    let ans = solve(n, &mut memo);

    println!("{}", ans.val());
}
