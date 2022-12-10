use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    if s.len() != 8 {
        println!("No");
        return;
    }

    if s[0].is_ascii_alphabetic()
        && s[7].is_ascii_alphabetic()
        && s[1..7].iter().all(|ch| ch.is_ascii_digit())
        && s[1] != '0'
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
