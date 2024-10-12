use std::mem;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut a: [Chars; n],
    };

    for i in 0..(n / 2) {
        let mut b = a.clone();
        for x in i..(n - i) {
            for y in i..(n - i) {
                b[y][n - x - 1] = a[x][y];
            }
        }
        mem::swap(&mut a, &mut b);
    }

    for row in a {
        println!("{}", row.iter().collect::<String>());
    }
}
