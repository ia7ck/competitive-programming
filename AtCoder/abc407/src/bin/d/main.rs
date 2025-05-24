use std::collections::HashSet;

use proconio::input;

const N: usize = 20;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[u64; w]; h],
    };

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            ans ^= a[i][j];
        }
    }
    let mut put = [[false; N]; N];
    let mut seen = HashSet::new();
    dfs(h, w, &a, ans, &mut ans, &mut put, &mut seen);
    println!("{}", ans);
}

fn dfs(h: usize, w: usize, a: &Vec<Vec<u64>>, acc: u64, ans: &mut u64, put: &mut [[bool; N]; N], seen: &mut HashSet<[[bool; N]; N]>) {
    if seen.contains(put) {
        return;
    }
    seen.insert(put.clone());
    *ans = (*ans).max(acc);
    for i in 0..h {
        for j in 0..w {
            // tate
            if i + 1 < h && !put[i][j] && !put[i + 1][j] {
                put[i][j] = true;
                put[i + 1][j] = true;
                dfs(h, w, a, acc ^ a[i][j] ^ a[i + 1][j], ans, put, seen);
                put[i][j] = false;
                put[i + 1][j] = false;
            }

            // yoko
            if j + 1 < w && !put[i][j] && !put[i][j + 1] {
                put[i][j] = true;
                put[i][j + 1] = true;
                dfs(h, w, a, acc ^ a[i][j] ^ a[i][j + 1], ans, put, seen);
                put[i][j] = false;
                put[i][j + 1] = false;
            }
        }
    }   
}
