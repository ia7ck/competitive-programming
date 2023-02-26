use std::collections::HashSet;

use proconio::{input, marker::Bytes};

fn main() {
    input! {
        _n: usize,
        s: Bytes,
    };

    let mut seen = HashSet::<(i32, i32)>::new();
    let (mut x, mut y) = (0, 0);
    seen.insert((x, y));
    for dir in s {
        match dir {
            b'R' => {
                x += 1;
            }
            b'L' => {
                x -= 1;
            }
            b'U' => {
                y += 1;
            }
            b'D' => {
                y -= 1;
            }
            _ => unreachable!(),
        }
        let new = seen.insert((x, y));
        if new == false {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
