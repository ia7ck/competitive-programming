use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        _n: usize,
        q: usize,
    };

    let m = 200000;
    let mut s = vec![BTreeSet::new(); m + 1];
    let mut t = vec![BTreeSet::new(); m + 1];

    for qq in 0..q {
        input! {
            op: u8,
        };

        if op == 1 {
            input! {
                i: usize,
                j: usize,
            };
            s[i].insert(j);
            t[j].insert((i, qq));
        } else if op == 2 {
            input! {
                i: usize,
            };
            for (k, (v, _)) in t[i].iter().enumerate() {
                print!("{}", v);
                if k + 1 < t[i].len() {
                    print!(" ");
                } else {
                    print!("\n");
                }
            }
        } else {
            input! {
                i: usize,
            };
            for (k, v) in s[i].iter().enumerate() {
                print!("{}", v);
                if k + 1 < s[i].len() {
                    print!(" ");
                } else {
                    print!("\n");
                }
            }
        }
    }
}
