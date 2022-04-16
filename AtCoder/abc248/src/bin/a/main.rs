use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    for c in "0123456789".chars() {
        if !s.contains(&c) {
            println!("{}", c);
            return;
        }
    }
    unreachable!();
}
