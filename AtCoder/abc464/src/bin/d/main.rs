use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            s: Chars,
            x: [i64; n],
            y: [i64; n - 1],
        };

        solve(n, s, x, y);
    }
}

fn solve(n: usize, s: Vec<char>, x: Vec<i64>, y: Vec<i64>) {
    let mut sun = 0;
    let mut rain = 0;
    if s[0] == 'S' {
        rain -= x[0];
    } else {
        sun -= x[0];
    }

    for i in 1..n {
        if s[i] == 'S' {
            (sun, rain) = (sun.max(rain + y[i - 1]), sun.max(rain) - x[i]);
        } else {
            (sun, rain) = (sun.max(rain + y[i - 1]) - x[i], sun.max(rain));
        }
    }

    println!("{}", sun.max(rain));
}
