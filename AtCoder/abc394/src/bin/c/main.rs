use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    };

    for i in (1..s.len()).rev() {
        if s[i - 1] == 'W' && s[i] == 'A' {
            s[i - 1] = 'A';
            s[i] = 'C';
        }
    }

    let ans = s.into_iter().collect::<String>();
    println!("{}", ans);
}
