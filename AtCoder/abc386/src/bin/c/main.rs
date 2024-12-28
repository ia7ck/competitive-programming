use proconio::{input, marker::Chars};

fn main() {
    input! {
        _k: usize,
        s: Chars,
        t: Chars,
    };

    if solve(&s, &t) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn solve(s: &Vec<char>, t: &Vec<char>) -> bool {
    if s.len() < t.len() {
        if s.len() + 1 != t.len() {
            return false;
        }
        let mut prefix = 0;
        for i in 0..s.len() {
            if s[i] == t[i] {
                prefix += 1;
            } else {
                break;
            }
        }
        let mut suffix = 0;
        for i in (0..s.len()).rev() {
            if s[i] == t[i + 1] {
                suffix += 1;
            } else {
                break;
            }
        }
        prefix + suffix >= s.len()
    } else if s.len() == t.len() {
        let mut diff = 0;
        for i in 0..s.len() {
            if s[i] != t[i] {
                diff += 1;
            }
        }
        diff <= 1
    } else {
        if s.len() != t.len() + 1 {
            return false;
        }
        let mut prefix = 0;
        for i in 0..t.len() {
            if s[i] == t[i] {
                prefix += 1;
            } else {
                break;
            }
        }
        let mut suffix = 0;
        for i in (0..t.len()).rev() {
            if s[i + 1] == t[i] {
                suffix += 1;
            } else {
                break;
            }
        }
        prefix + suffix >= t.len()
    }
}
