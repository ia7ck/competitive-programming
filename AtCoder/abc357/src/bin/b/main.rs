use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let lower = s.iter().filter(|&&c| c.is_lowercase()).count();
    let upper = s.len() - lower;
    if lower < upper {
        for ch in s {
            print!("{}", ch.to_ascii_uppercase());
        }
    } else {
        for ch in s {
            print!("{}", ch.to_ascii_lowercase());
        }
    }
    println!();
}
