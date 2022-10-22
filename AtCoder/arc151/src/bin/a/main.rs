use proconio::input;
use proconio::marker::Chars;

fn dist(n: usize, x: &[char], y: &[char]) -> usize {
    let mut d = 0;
    for i in 0..n {
        if x[i] != y[i] {
            d += 1;
        }
    }
    d
}

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    };

    let u: Vec<char> = "0".repeat(n).chars().collect();
    let (s, t) = if dist(n, &s, &u) <= dist(n, &t, &u) {
        (s, t)
    } else {
        (t, s)
    };
    let (mut a, mut b) = (dist(n, &s, &u), dist(n, &t, &u));
    assert!(a <= b);
    if (b - a) % 2 == 1 {
        println!("-1");
        return;
    }
    let mut u = u;
    for i in (0..n).rev() {
        if a < b && s[i] == '0' && t[i] == '1' {
            u[i] = '1';
            a += 1;
            b -= 1;
        }
    }
    assert_eq!(a, b);
    for ch in u {
        print!("{}", ch);
    }
    println!();
}
