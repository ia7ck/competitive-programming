use next_permutation::NextPermutation;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        k: usize,
        mut s: Chars,
    };

    s.sort();
    let mut ans = 0;
    loop {
        let mut found = false;
        for t in s.windows(k) {
            if is_palindrome(t) {
                found = true;
            }
        }
        if !found {
            ans += 1;
        }
        if !s.next_permutation() {
            break;
        }
    }
    println!("{}", ans);
}

fn is_palindrome(t: &[char]) -> bool {
    t.iter().zip(t.iter().rev()).all(|(c1, c2)| c1 == c2)
}
