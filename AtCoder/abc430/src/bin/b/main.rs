use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    };

    let mut ans = HashSet::new();
    for i in 0..=(n - m) {
        for j in 0..=(n - m) {
            let mut t = vec![vec!['.'; m]; m];
            for y in i..(i + m) {
                for x in j..(j + m) {
                    t[y - i][x - j] = s[y][x];
                }
            }
            ans.insert(t);
        }
    }

    println!("{}", ans.len());
}
