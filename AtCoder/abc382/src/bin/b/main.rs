use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut d: usize,
        mut s: Chars,
    };

    for i in (0..n).rev() {
        if d >= 1 && s[i] == '@' {
            d -= 1;
            s[i] = '.';
        }
    }

    println!("{}", s.iter().collect::<String>());
}
