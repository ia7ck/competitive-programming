use proconio::input;
use proconio::marker::Chars;

fn f(s: &[char]) -> Vec<(char, usize)> {
    let mut ch = s[0];
    let mut len = 1;
    let mut result = Vec::new();
    for &c in &s[1..] {
        if ch != c {
            result.push((ch, len));
            ch = c;
            len = 1;
        } else {
            len += 1;
        }
    }
    result.push((ch, len));
    result
}

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };

    let u = f(&s);
    let v = f(&t);

    if u.len() != v.len() {
        println!("No");
        return;
    }

    for (&(c, n), &(d, m)) in u.iter().zip(&v) {
        if c != d {
            println!("No");
            return;
        }
        if n == 1 && m != 1 {
            println!("No");
            return;
        }
        if n > m {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
