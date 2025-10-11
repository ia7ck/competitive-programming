use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut ans = String::new();
    for i in 0..s.len() {
        if i != s.len() / 2 {
            ans.push(s[i]);
        }
    }
    println!("{}", ans);
}
