use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: u64,
        a: u64,
        x: u64,
        y: u64,
    }

    let mut memo = HashMap::new();
    let ans = solve(n, a, x, y, &mut memo);
    println!("{}", ans);
}

fn solve(n: u64, a: u64, x: u64, y: u64, memo: &mut HashMap<u64, f64>) -> f64 {
    if n == 0 {
        return 0.0;
    }
    if let Some(ans) = memo.get(&n) {
        return *ans;
    }
    let res1 = x as f64 + solve(n / a, a, x, y, memo);
    let mut res2 = (y * 6) as f64 / 5 as f64;
    for i in 2..=6 {
        res2 += solve(n / i, a, x, y, memo) / 5.0;
    }

    let ans = res1.min(res2);
    memo.insert(n, ans);
    ans
}
