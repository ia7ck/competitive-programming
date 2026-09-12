use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut ans = String::new();
    for i in 0..s.len() {
        ans.push(s[i]);
        if i + 1 < s.len() {
            ans.push('o');
        }
    }

    println!("{}", ans);
}
