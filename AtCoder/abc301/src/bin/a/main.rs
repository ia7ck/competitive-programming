use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let t = s.iter().filter(|&&ch| ch == 'T').count();
    let a = s.iter().filter(|&&ch| ch == 'A').count();

    if t > a {
        println!("T");
    } else if t < a {
        println!("A");
    } else if s[n - 1] == 'T' {
        println!("A");
    } else {
        println!("T");
    }
}
