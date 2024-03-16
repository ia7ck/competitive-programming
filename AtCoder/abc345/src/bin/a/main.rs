use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let ok = s[0] == '<' && s[1..(s.len() - 1)].iter().all(|&ch| ch == '=') && s[s.len() - 1] == '>';
    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
