use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    };

    let mut visited = HashSet::new();

    let (mut x, mut y) = (0, 0);
    visited.insert((x, y));
    for c in s {
        match c {
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
        if visited.insert((x, y)) {
            // new
        } else {
            println!("Yes");
            return;
        }
    }

    println!("No");
}