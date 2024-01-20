use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let rest = s
        .iter()
        .skip_while(|&&ch| ch == 'A')
        .skip_while(|&&ch| ch == 'B')
        .skip_while(|&&ch| ch == 'C');
    if rest.count() == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
