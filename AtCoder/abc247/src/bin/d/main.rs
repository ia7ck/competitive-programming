use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        q: usize,
    };
    let mut deq = VecDeque::new();
    for _ in 0..q {
        input! {
            t: u8,
        };
        if t == 1 {
            input! {
                x: u64,
                c: u64,
            };
            deq.push_back((x, c));
        } else if t == 2 {
            input! {
                mut c: u64,
            };
            let mut ans = 0;
            while let Some((x, d)) = deq.pop_front() {
                if c >= d {
                    ans += x * d;
                    c -= d;
                } else {
                    ans += x * c;
                    deq.push_front((x, d - c));
                    c = 0;
                    break;
                }
            }
            assert_eq!(c, 0);
            println!("{}", ans);
        } else {
            unreachable!();
        }
    }
}
