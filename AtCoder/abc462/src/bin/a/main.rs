use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    for ch in s.into_iter().filter(|&ch| ch.is_ascii_digit()) {
        print!("{}", ch);
    }

    println!();
}
