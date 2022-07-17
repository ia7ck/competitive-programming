use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    };

    s.sort();
    if s[0] == s[1] && s[1] == s[2] {
        println!("-1");
    } else if s[0] == s[1] && s[1] < s[2] {
        println!("{}", s[2]);
    } else {
        assert!(s[0] < s[1]);
        println!("{}", s[0]);
    }
}
