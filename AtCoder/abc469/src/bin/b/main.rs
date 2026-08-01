use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: Chars,
    };

    s.insert(0, 'x');
    s.push('x');

    let mut ans = 0;
    for i in 1..=n {
        if s[i - 1] == 'x' && s[i] == 'x' && s[i + 1] == 'x' {
            ans += 1;
        }
    }

    println!("{}", ans);
}
