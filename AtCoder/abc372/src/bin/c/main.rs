use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        queries: [(Usize1, char); q],
    };

    let mut ans = 0;
    for i in 1..(n - 1) {
        if s[(i - 1)..=(i + 1)] == ['A', 'B', 'C'] {
            ans += 1;
        }
    }
    let mut s = s;
    for (i, c) in queries {
        if s[i] == 'A' {
            if i + 2 < n && c != 'A' && s[i + 1] == 'B' && s[i + 2] == 'C' {
                dbg!("here");
                assert!(ans >= 1);
                ans -= 1;
            }
        } else {
            if i + 2 < n && c == 'A' && s[i + 1] == 'B' && s[i + 2] == 'C' {
                ans += 1;
            }
        }
        if s[i] == 'B' {
            if i >= 1 && i + 1 < n && s[i - 1] == 'A' && c != 'B' && s[i + 1] == 'C' {
                assert!(ans >= 1);
                ans -= 1;
            }
        } else {
            if i >= 1 && i + 1 < n && s[i - 1] == 'A' && c == 'B' && s[i + 1] == 'C' {
                ans += 1;
            }
        }
        if s[i] == 'C' {
            if i >= 2 && s[i - 2] == 'A' && s[i - 1] == 'B' && c != 'C' {
                assert!(ans >= 1);
                ans -= 1;
            }
        } else {
            if i >= 2 && s[i - 2] == 'A' && s[i - 1] == 'B' && c == 'C' {
                ans += 1;
            }
        }
        s[i] = c;
        println!("{}", ans);
    }
}
