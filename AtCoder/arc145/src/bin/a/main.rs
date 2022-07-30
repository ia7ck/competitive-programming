use proconio::input;
use proconio::marker::Chars;

fn palindrome(s: &[char]) -> bool {
    for l in 0..s.len() / 2 {
        let r = s.len() - l - 1;
        if s[l] != s[r] {
            return false;
        }
    }
    true
}

fn main() {
    input! {
        n: usize,
        mut s: Chars,
    };

    if palindrome(&s) {
        println!("Yes");
        return;
    }
    if n == 2 {
        // AB, BA
        println!("No");
        return;
    }
    if s[0] == 'A' && s[n - 1] == 'B' {
        println!("No");
    } else {
        println!("Yes");
    }
}
