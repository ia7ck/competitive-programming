use proconio::{input, marker::{Chars, Usize1}};

fn main() {
    input! {
        _n: usize,
        l: Usize1,
        r: Usize1,
        s: Chars,
    };

    if s[l..=r].iter().all(|&c| c == 'o') {
        println!("Yes");
    } else {
        println!("No");
    }
}
