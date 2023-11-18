use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let ans = s
        .into_iter()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", ans);
}
