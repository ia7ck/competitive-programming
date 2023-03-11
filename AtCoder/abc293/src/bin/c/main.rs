use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[u32; w]; h],
    };

    let mut ans = 0;
    let mut que = VecDeque::new();
    que.push_back((0, 0, vec![a[0][0]]));
    while let Some((i, j, v)) = que.pop_front() {
        if i == h - 1 && j == w - 1 {
            let n = v.len();
            let mut v = v;
            v.sort();
            v.dedup();
            if v.len() == n {
                ans += 1;
            }
            continue;
        }
        if i + 1 < h {
            let mut vv = v.clone();
            vv.push(a[i + 1][j]);
            que.push_back((i + 1, j, vv));
        }
        if j + 1 < w {
            let mut vv = v.clone();
            vv.push(a[i][j + 1]);
            que.push_back((i, j + 1, vv));
        }
    }

    println!("{}", ans);
}
