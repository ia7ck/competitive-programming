use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let ans = if n % 2 == 0 {
        false
    } else {
        let k = (n + 1) / 2;
        s[..(k - 1)].iter().all(|c| c == &'1')
            && s[k - 1] == '/'
            && s[k..].iter().all(|c| c == &'2')
    };

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
