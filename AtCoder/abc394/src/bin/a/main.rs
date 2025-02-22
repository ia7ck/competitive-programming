use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let ans = s.into_iter().filter(|&c| c == '2').collect::<String>();
    println!("{}", ans);
}
