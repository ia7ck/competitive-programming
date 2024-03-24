use std::collections::BTreeSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: u64,
        b: u64,
        s: Chars,
    };

    let mut ans = 0;

    let mut s = s;
    let open = s.iter().filter(|&&ch| ch == '(').count();
    let close = s.iter().filter(|&&ch| ch == ')').count();
    if open > close {
        let mut diff = open - close;
        assert_eq!(diff % 2, 0);
        for i in (0..(n * 2)).rev() {
            if s[i] == '(' {
                s[i] = ')';
                ans += b;
                diff -= 2;
                if diff == 0 {
                    break;
                }
            }
        }
    } else if open < close {
        let mut diff = close - open;
        assert_eq!(diff % 2, 0);
        for i in 0..(n * 2) {
            if s[i] == ')' {
                s[i] = '(';
                ans += b;
                diff -= 2;
                if diff == 0 {
                    break;
                }
            }
        }
    }

    let mut open_pos = BTreeSet::new();
    for i in 0..(n * 2) {
        if s[i] == '(' {
            open_pos.insert(i);
        }
    }

    let mut open = 0_isize;
    for i in 0..(n * 2) {
        if s[i] == '(' {
            open += 1;
            open_pos.remove(&i);
        } else {
            if open == 0 {
                assert_ne!(i, n * 2 - 1);
                if a >= b * 2 {
                    s[i] = '(';
                    ans += b;
                } else if let Some(j) = open_pos.pop_last() {
                    assert!(i < j);
                    assert_eq!(s[j], '(');
                    s.swap(i, j);
                    ans += a;
                } else {
                    s[i] = '(';
                    ans += b;
                }
                open += 1;
            } else {
                open -= 1;
            }
        }
    }
    assert!(open >= 0);
    assert_eq!(open % 2, 0);
    ans += (open / 2) as u64 * b;
    println!("{}", ans);
}
