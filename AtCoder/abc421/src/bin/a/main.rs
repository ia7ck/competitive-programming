use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        s: [String; n],
        x: Usize1,
        y: String,
    };

    if s[x] == y {
        println!("Yes");
    } else {
        println!("No");
    }
}
