use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    };

    let ans = if n[0] == n[1] && n[1] == n[2] {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
