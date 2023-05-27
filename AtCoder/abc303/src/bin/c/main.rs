use std::{collections::HashSet, unreachable};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut h: i32,
        k: i32,
        s: Chars,
        xy: [(i32, i32); m],
    };

    let mut xy_set = xy.into_iter().collect::<HashSet<_>>();
    let (mut x, mut y) = (0, 0);
    for i in 0..n {
        h -= 1;
        match s[i] {
            'R' => {
                x += 1;
            }
            'L' => {
                x -= 1;
            }
            'U' => {
                y += 1;
            }
            'D' => {
                y -= 1;
            }
            _ => unreachable!(),
        }
        if h < 0 {
            println!("No");
            return;
        }
        if h < k && xy_set.contains(&(x, y)) {
            h = k;
            xy_set.remove(&(x, y));
        }
    }

    println!("Yes");
}
