use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        t: i64,
        s: Chars,
        x: [i64; n],
    };

    let ans = solve(n, t, &s, &x);
    println!("{}", ans);
}

fn solve(n: usize, t: i64, s: &Vec<char>, x: &Vec<i64>) -> usize {
    let mut ord = (0..n).collect::<Vec<usize>>();
    ord.sort_by_key(|&i| x[i]);

    let s = {
        let mut s_ = Vec::new();
        for &i in &ord {
            s_.push(s[i]);
        }
        s_
    };
    let x = {
        let mut x_ = Vec::new();
        for &i in &ord {
            x_.push(x[i]);
        }
        x_
    };

    let mut left = Vec::new();
    for i in 0..n {
        if s[i] == '0' {
            left.push(x[i]);
        }
    }

    let mut l = 0;
    let mut ans = 0;
    for i in 0..n {
        if s[i] == '0' {
            l += 1;
        } else {
            let p = left.partition_point(|&y| y <= x[i] + t * 2);
            assert!(p >= l);
            ans += p - l;
        }
    }

    ans
}
