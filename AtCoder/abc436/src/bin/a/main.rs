use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: Chars,
    };

    while s.len() < n {
        s.insert(0, 'o');
    }

    println!("{}", s.iter().collect::<String>());
}
