use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        q: usize,
    };

    let mut que = VecDeque::new();
    for _ in 0..q {
        input! {
            op: u8,
        };

        if op == 1 {
            input! {
                c: usize,
                x: u64,
            };
            que.push_back((c, x));
        } else {
            input! {
                mut k: usize,
            };
            let mut ans = 0;
            while k > 0 {
                let (c, x) = que.pop_front().unwrap();
                if k >= c {
                    k -= c;
                    ans += (c as u64) * x;
                } else {
                    ans += (k as u64) * x;
                    que.push_front((c - k, x));
                    break;
                }
            }
            println!("{}", ans);
        }
    }
}
