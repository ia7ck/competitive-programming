extern crate proconio;
use proconio::input;

fn main() {
    input! {
      mut s: String,
    }
    let s = s.chars().collect::<Vec<_>>();
    let mut ans = 0;
    if s[0] == 'R' || s[1] == 'R' || s[2] == 'R' {
        ans = 1;
    }
    if s[0..2] == ['R', 'R'] || s[1..3] == ['R', 'R'] {
        ans = 2;
    }
    if s == ['R', 'R', 'R'] {
        ans = 3;
    }
    println!("{}", ans);
}
