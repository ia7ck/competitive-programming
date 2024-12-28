use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        xyc: [(usize, usize, char); m],
    };

    let yxc = xyc.iter().map(|&(x, y, c)| (y, x, c)).collect::<Vec<_>>();

    if solve(n, &xyc) && solve(n, &yxc) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn solve(n: usize, xyc: &Vec<(usize, usize, char)>) -> bool {
    let mut yc_by_x = BTreeMap::new();
    for &(x, y, c) in xyc {
        yc_by_x.entry(x).or_insert(Vec::new()).push((y, c));
    }
    let mut right = n;
    for (_x, mut yc) in yc_by_x {
        yc.sort_unstable();
        for (y, c) in yc {
            if c == 'B' {
                if right < y {
                    return false;
                }
            } else {
                right = right.min(y - 1);
            }
        }
    }

    true
}
