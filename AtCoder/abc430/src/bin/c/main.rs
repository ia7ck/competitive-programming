use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: isize,
        b: isize,
        s: Chars,
    };

    let mut ca = vec![0; n + 1];
    let mut cb = vec![0; n + 1];
    for i in 0..n {
        ca[i + 1] = ca[i];
        cb[i + 1] = cb[i];
        if s[i] == 'a' {
            ca[i + 1] += 1;
        } else {
            cb[i + 1] += 1;
        }
    }

    let mut ans = 0;
    for r in 0..n {
        let p = ca.partition_point(|&x| x <= ca[r + 1] - a);
        let q = cb.partition_point(|&x| x <= cb[r + 1] - b);
        if q < p {
            ans += p - q;
        }
    }

    println!("{}", ans);
}
