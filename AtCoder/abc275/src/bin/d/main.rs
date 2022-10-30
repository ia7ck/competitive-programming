use std::collections::HashMap;

use proconio::input;

fn solve(k: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if k == 0 {
        return 1;
    }

    if let Some(&ans) = memo.get(&k) {
        return ans;
    }

    let ans = solve(k / 2, memo) + solve(k / 3, memo);
    memo.insert(k, ans);
    ans
}

fn main() {
    input! {
        n: usize,
    };

    let mut memo = HashMap::new();
    let ans = solve(n, &mut memo);
    println!("{}", ans);
}
