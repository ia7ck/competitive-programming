use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            s: Chars,
        };

        solve(n, s);
    }
}

fn solve(n: usize, s: Vec<char>) {
    let mut memo = HashMap::new();
    let ans = f((1 << n) - 1, n, &s, &mut memo);
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn f(state: usize, n: usize, s: &Vec<char>, memo: &mut HashMap<usize, bool>) -> bool {
    if state == 0 {
        return true;
    }

    if s[state - 1] == '1' {
        return false;
    }

    if let Some(ans) = memo.get(&state) {
        return *ans;
    }

    let mut ans = false;
    for i in 0..n {
        if state >> i & 1 == 1 {
            let new_state = state ^ (1 << i);
            if f(new_state, n, s, memo) {
                ans = true;
            }
        }
    }

    memo.insert(state, ans);
    ans
}
