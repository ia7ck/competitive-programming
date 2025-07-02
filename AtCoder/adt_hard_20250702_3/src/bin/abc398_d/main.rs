use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        r: i32,
        c: i32,
        s: Chars,
    };

    let mut ans = String::new();
    let (mut r, mut c) = (r, c);
    let (mut x, mut y) = (0, 0);
    let mut set = HashSet::from([(x, y)]);
    for d in s {
        match d {
            'N' => {
                r += 1;
                x += 1;
            }
            'W' => {
                c += 1;
                y += 1;
            }
            'S' => {
                r -= 1;
                x -= 1;
            }
            'E' => {
                c -= 1;
                y -= 1;
            }
            _ => unreachable!(),
        }
        if set.contains(&(r, c)) {
            ans.push('1');
        } else {
            ans.push('0');
        }
        set.insert((x, y));
    }

    println!("{}", ans);
}
