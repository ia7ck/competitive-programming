use proconio::{input, marker::Chars};

fn palindrome(s: &Vec<char>) -> bool {
    let mut t = s.clone();
    t.reverse();
    t == *s
}

fn main() {
    input! {
        s: Chars,
    };

    let mut ans = 1;
    for i in 0..s.len() {
        for j in (i + 1)..=s.len() {
            let ss = s[i..j].to_vec();
            if palindrome(&ss) {
                ans = ans.max(ss.len());
            }
        }
    }
    println!("{}", ans);
}
