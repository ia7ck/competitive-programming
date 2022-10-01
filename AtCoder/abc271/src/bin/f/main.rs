use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
    };

    let mut freq = vec![vec![HashMap::new(); n]; n];
    freq[0][0].insert(a[0][0], 1);
    for i in 0..n {
        for j in 0..=(n - i - 1) {
            if (i, j) == (0, 0) {
                continue;
            }
            let mut f = HashMap::new();
            if i >= 1 {
                for (&k, &v) in &freq[i - 1][j] {
                    *f.entry(k ^ a[i][j]).or_insert(0) += v;
                }
            }
            if j >= 1 {
                for (&k, &v) in &freq[i][j - 1] {
                    *f.entry(k ^ a[i][j]).or_insert(0) += v;
                }
            }
            freq[i][j] = f;
        }
    }
    let mut freq2 = vec![vec![HashMap::new(); n]; n];
    freq2[n - 1][n - 1].insert(a[n - 1][n - 1], 1);
    for i in (0..n).rev() {
        for j in ((n - i - 1)..n).rev() {
            if (i, j) == (n - 1, n - 1) {
                continue;
            }
            let mut f = HashMap::new();
            if i + 1 < n {
                for (&k, &v) in &freq2[i + 1][j] {
                    *f.entry(k ^ a[i][j]).or_insert(0) += v;
                }
            }
            if j + 1 < n {
                for (&k, &v) in &freq2[i][j + 1] {
                    *f.entry(k ^ a[i][j]).or_insert(0) += v;
                }
            }
            freq2[i][j] = f;
        }
    }
    let mut ans = 0_u64;
    for i in 0..n {
        let j = n - i - 1;
        for (&k, &v) in &freq[i][j] {
            if let Some(&v2) = freq2[i][j].get(&(k ^ a[i][j])) {
                ans += v * v2;
            }
        }
    }
    println!("{}", ans);
}
