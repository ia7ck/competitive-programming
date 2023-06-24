use proconio::{input, marker::Chars};

fn check(s: &Vec<char>) -> bool {
    for i in 0..s.len() {
        if s[i] != s[s.len() - i - 1] {
            return false;
        }
    }
    true
}

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let mut t = Vec::new();
            for &c in &s[i] {
                t.push(c);
            }
            for &c in &s[j] {
                t.push(c);
            }
            if check(&t) {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
