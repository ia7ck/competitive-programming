use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    };

    let mut a = false;
    let mut b = false;
    let mut c = false;
    for (i, &x) in s.iter().enumerate() {
        if x == 'A' {
            a = true;
        } else if x == 'B' {
            b = true;
        } else {
            c = true;
        }
        if a && b && c {
            println!("{}", i + 1);
            return;
        }
    }
    unreachable!();
}
