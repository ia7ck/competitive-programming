use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        events: [(u8, Usize1); n],
    };

    let mut time = vec![BTreeSet::new(); n];
    let mut pick = vec![false; n];
    for (i, &(op, x)) in events.iter().enumerate() {
        if op == 1 {
            time[x].insert(i);
        } else {
            if let Some(last) = time[x].pop_last() {
                pick[last] = true;
            } else {
                println!("-1");
                return;
            }
        }
    }

    let mut ans_k = 0;
    let mut cur = 0;
    let mut ans = Vec::new();
    for (i, &(op, _)) in events.iter().enumerate() {
        if op == 1 {
            if pick[i] {
                cur += 1;
                ans.push("1");
            } else {
                ans.push("0");
            }
        } else {
            assert!(cur >= 1);
            cur -= 1;
        }
        ans_k = ans_k.max(cur);
    }
    println!("{}", ans_k);
    println!("{}", ans.join(" "));
}
