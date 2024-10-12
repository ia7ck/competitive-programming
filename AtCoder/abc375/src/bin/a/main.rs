use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let mut ans = 0;
    for i in 1..(n - 1) {
        if s[i - 1] == '#' && s[i] == '.' && s[i + 1] == '#' {
            ans += 1;
        }
    }
    println!("{}", ans);
}
