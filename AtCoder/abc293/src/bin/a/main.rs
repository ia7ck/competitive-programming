use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    };

    for i in 0..(s.len() / 2) {
        s.swap(i * 2, i * 2 + 1);
    }

    for b in s {
        print!("{}", b);
    }
    println!();
}
