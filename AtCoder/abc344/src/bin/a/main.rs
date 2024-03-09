use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let l = s.iter().position(|&ch| ch == '|').unwrap();
    let r = s.iter().rposition(|&ch| ch == '|').unwrap();
    let mut ans = String::new();
    for i in 0..l {
        ans.push(s[i]);
    }
    for i in (r + 1)..s.len() {
        ans.push(s[i]);
    }
    println!("{}", ans);
}
