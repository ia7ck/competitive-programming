use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: u64,
        y: u64,
        z: u64,
        s: Chars,
    };

    let (mut on, mut off) = if s[0] == 'a' {
        (z + y, x)
    } else {
        (z + x, y)
    };
    for i in 1..s.len() {
        let (on_, off_) = if s[i] == 'a' {
            ((off + (z + y)).min(on + y), (off + x).min(on + (z + x)))
        } else {
            ((off + (z + x)).min(on + x), (off + y).min(on + (z + y)))
        };
        on = on_;
        off = off_;
    }

    println!("{}", on.min(off));
}
