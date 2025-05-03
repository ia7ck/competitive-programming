use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    for ch in 'a'..='z' {
        if !s.contains(&ch) {
            println!("{}", ch);
            return;
        }
    }

    unreachable!();
}
