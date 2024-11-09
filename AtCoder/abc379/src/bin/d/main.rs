use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        q: usize,
    };

    let mut heap = BinaryHeap::new();
    let mut t_sum = 0;
    for _ in 0..q {
        input! {
            op: u8,
        };
        if op == 1 {
            heap.push(-t_sum);
        } else if op == 2 {
            input! {
                t: i64,
            };
            t_sum += t;
        } else {
            input! {
                h: i64,
            };
            let mut ans = 0;
            while let Some(&x) = heap.peek() {
                if x + t_sum >= h {
                    ans += 1;
                    heap.pop(); // x
                } else {
                    break;
                }
            }
            println!("{}", ans);
        }
    }
}
