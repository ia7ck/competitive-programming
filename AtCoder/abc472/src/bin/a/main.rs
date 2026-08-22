use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut ans = String::new();
    for c in s {
        if c == 'A' {
            ans.push('A');
        } else {
            ans.push('.');
        }
    }

    println!("{}", ans);
}
