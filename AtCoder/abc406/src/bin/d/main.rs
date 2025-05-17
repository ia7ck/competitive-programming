use std::collections::HashSet;
use std::mem;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        xy: [(Usize1, Usize1); n],
        q: usize,
    };

    let mut rows = vec![HashSet::new(); h];
    let mut cols = vec![HashSet::new(); w];
    for (x, y) in xy {
        rows[x].insert(y);
        cols[y].insert(x);
    }

    for _ in 0..q {
        input! {
            op: u8,
        };
        if op == 1 {
            input! {
                x: Usize1,
            };
            let ans = rows[x].len();
            for y in mem::take(&mut rows[x]) {
                cols[y].remove(&x);
            }
            println!("{}", ans);
        } else {
            input! {
                y: Usize1,
            };
            let ans = cols[y].len();
            for x in mem::take(&mut cols[y]) {
                rows[x].remove(&y);
            }
            println!("{}", ans);
        }
    }
}
