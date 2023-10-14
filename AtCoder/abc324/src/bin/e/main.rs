use proconio::{input, marker::Chars};
use binary_search_range::BinarySearchRange;

fn main() {
    input! {
        n: usize,
        t: Chars,
        ss: [Chars; n],
    };

    let mut next = Vec::new();
    for i in 0..n {
        let mut next_ = vec![[None; 26]; ss[i].len()];
        for j in (0..(ss[i].len() - 1)).rev() {
            next_[j] = next_[j + 1];
            for k in 0..26 {
                if usize::from(ss[i][j + 1] as u8 - b'a') == k {
                    next_[j][k] = Some(j + 1);
                }
            }
        }
        next.push(next_);
    }

    let mut prev = Vec::new();
    for i in 0..n {
        let mut prev_ = vec![[None; 26]; ss[i].len()];
        for j in 1..ss[i].len() {
            prev_[j] = prev_[j - 1];
            for k in 0..26 {
                if usize::from(ss[i][j - 1] as u8 - b'a') == k {
                    prev_[j][k] = Some(j - 1);
                }
            }
        }
        prev.push(prev_);
    }

    let mut a = vec![0; n];
    for i in 0..n {
        let mut j = 0;
        for k in 0..t.len() {
            if j >= ss[i].len() {
                break;
            }
            if ss[i][j] == t[k] {
                a[i] += 1;
                j += 1;
            } else if let Some(new_j) = next[i][j][usize::from(t[k] as u8 - b'a')] {
                a[i] += 1;
                j = new_j + 1;
            } else {
                break;
            }
        }
    }
    let mut b = vec![0; n];
    for i in 0..n {
        let mut j = ss[i].len() - 1;
        for k in (0..t.len()).rev() {
            if ss[i][j] == t[k] {
                b[i] += 1;
                if j == 0 {
                    break;
                } else {
                    j -= 1;
                }
            } else if let Some(new_j) = prev[i][j][usize::from(t[k] as u8 - b'a')] {
                b[i] += 1;
                j = new_j;
                if j == 0 {
                    break;
                } else {
                    j -= 1;
                }
            } else {
                break;
            }
        }
    }

    let mut ans = 0_usize;
    b.sort();
    for x in a {
        // x + b[i] >= t.len()
        let rng = b.range(t.len().saturating_sub(x)..(t.len() + 1));
        ans += rng.len();
    }
    println!("{}", ans);
}
