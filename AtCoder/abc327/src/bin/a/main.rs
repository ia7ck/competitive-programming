use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    };

    for w in s.windows(2) {
        if w == &['a', 'b'] || w == &['b', 'a'] {
            println!("Yes");
            return;
        } 
    }
    println!("No");
}
