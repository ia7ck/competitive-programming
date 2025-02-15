use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let mut ones = Vec::new();
    for i in 0..n {
        if s[i] == '1' {
            ones.push(i);
        }
    }
    let len = ones.len();
    let ans = if len % 2 == 1 {
        let m = ones[len / 2];
        assert!(m >= len / 2);
        assert!(m + len / 2 < n);
        count(&s, m - len / 2, m + len / 2 + 1)
    } else {
        let mut ans = usize::MAX;
        for m in [ones[len / 2 - 1], ones[len / 2]] {
            if m >= len / 2 - 1 && m + len / 2 < n {
                ans = count(&s, m - (len / 2 - 1), m + len / 2 + 1);
            }
            if m >= len / 2 && m + (len / 2 - 1) < n {
                ans = ans.min(count(&s, m - len / 2, m + len / 2));
            }
        }
        assert_ne!(ans, usize::MAX);
        ans
    };

    println!("{}", ans);
}

fn count(s: &Vec<char>, l: usize, r: usize) -> usize {
    let mut ones = Vec::new();
    for i in 0..s.len() {
        if s[i] == '1' {
            ones.push(i);
        }
    }
    assert_eq!(ones.len(), r - l);
    let mut ans = 0;
    for i in l..r {
        ans += i.abs_diff(ones[i - l]);
    }
    ans
}

// 0101001001
//     1111
