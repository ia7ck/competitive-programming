use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        c: char,
    };

    let mut ans = String::new();
    for x in s {
        ans.push(x);
        if x == c {
            ans.push(x);
        }
    }
    println!("{}", ans);
}
