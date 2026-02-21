use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    };

    s[0] = s[0].to_ascii_lowercase();

    print!("Of");
    for ch in s {
        print!("{}", ch);
    }
    println!();
}
