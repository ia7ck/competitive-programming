use proconio::{input, marker::Chars};

fn main() {
    let alph = "abcdefghijklmnopqrstuvwxyz";
    let mut dict = Vec::new();
    for c in alph.chars() {
        for d in alph.chars() {
            if c != d {
                dict.push(vec![c, d]);
            }
        }
    }
    input! {
        t: usize,
    };
    for _ in 0..t {
        input! {
            s: Chars,
        };
        let ans = dict.iter().position(|w| w == &s).unwrap();
        println!("{}", ans + 1);
    }
}
