use std::collections::HashMap;

use proconio::input;

fn solve(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if n <= 1 {
        return 0;
    }
    if let Some(&v) = memo.get(&n) {
        return v;
    }
    let ans = if n % 2 == 0 {
        n + solve(n / 2, memo) * 2
    } else {
        n + solve((n - 1) / 2, memo) + solve((n + 1) / 2, memo)
    };
    memo.insert(n, ans);
    ans
}

fn main() {
    input! {
        n: u64,
    };

    let mut memo = HashMap::new();
    let ans = solve(n, &mut memo);
    println!("{}", ans);
}
