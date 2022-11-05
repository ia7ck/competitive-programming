use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    if let Some(ans) = s.iter().rposition(|&ch| ch == 'a') {
        println!("{}", ans + 1);
    } else {
        println!("-1");
    }
}
