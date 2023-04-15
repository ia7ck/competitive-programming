use std::collections::VecDeque;

use proconio::input;

fn mpow(a: u64, x: u64, m: u64) -> u64 {
    if x == 0 {
        1 % m
    } else if x == 1 {
        a % m
    } else if x % 2 == 0 {
        mpow(a * a % m, x / 2, m)
    } else {
        a * mpow(a, x - 1, m) % m
    }
}

fn main() {
    input! {
        q: usize,
    };

    const M: u64 = 998244353;
    let mut s = VecDeque::new();
    s.push_back(1);
    let mut ans = 1;
    for _ in 0..q {
        input! {
            op: u8,
        };

        if op == 1 {
            input! {
                x: u64,
            };
            s.push_back(x);
            ans = (ans * 10 + x) % M;
        } else if op == 2 {
            let l = s.len() as u64;
            let y = s.pop_front().unwrap();
            ans = (M + ans - y * mpow(10, l - 1, M) % M) % M;
        } else {
            println!("{}", ans);
        }
    }
}
