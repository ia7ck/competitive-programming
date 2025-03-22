use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        r: i64,
        c: i64,
        s: Chars,
    };

    let ord = {
        let mut ord = HashMap::new();
        let (mut y, mut x) = (0_i64, 0_i64);
        for i in (0..n).rev() {
            match s[i] {
                'N' => y -= 1,
                'W' => x -= 1,
                'S' => y += 1,
                'E' => x += 1,
                _ => unreachable!(),
            }
            ord.insert((y, x), i);
        }
        ord
    };

    let mut ans = Vec::new();
    let (mut r, mut c) = (r, c);
    for i in (0..n).rev() {
        match ord.get(&(r, c)) {
            Some(&j) if j <= i => {
                ans.push('1');
            }
            _ => {
                ans.push('0');
            }
        }
        match s[i] {
            'N' => r -= 1,
            'W' => c -= 1,
            'S' => r += 1,
            'E' => c += 1,
            _ => unreachable!(),
        }
    }
    ans.reverse();
    println!("{}", ans.into_iter().collect::<String>());
}
