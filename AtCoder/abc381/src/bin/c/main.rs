use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let mut left1 = vec![0; n];
    let mut right2 = vec![0; n];
    for i in 0..n {
        if i >= 1 {
            left1[i] = left1[i - 1];
        }
        if s[i] == '1' {
            left1[i] += 1;
        } else {
            left1[i] = 0;
        }
    }
    for i in (0..n).rev() {
        if i + 1 < n {
            right2[i] = right2[i + 1];
        }
        if s[i] == '2' {
            right2[i] += 1;
        } else {
            right2[i] = 0;
        }
    }

    let mut ans = 0;
    for i in 0..n {
        if s[i] == '/' {
            let l = if i == 0 { 0 } else { left1[i - 1] };
            let r = if i + 1 == n { 0 } else { right2[i + 1] };
            ans = ans.max(l.min(r) * 2 + 1);
        }
    }

    println!("{}", ans);
}
