use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        _m: usize,
        q: usize,
    };

    let mut all = vec![false; n];
    let mut individual = vec![HashSet::<usize>::new(); n];

    for _ in 0..q {
        input! {
            op: u8,
        };
        if op == 1 {
            input! {
                x: Usize1,
                y: Usize1,
            };
            individual[x].insert(y);
        } else if op == 2 {
            input! {
                x: Usize1,
            };
            all[x] = true;
        } else {
            input! {
                x: Usize1,
                y: Usize1,
            };
            let ans = all[x] || individual[x].contains(&y);
            if ans {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
