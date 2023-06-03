use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i32,
        xy: [(i32, i32); n],
    };

    let mut g = vec![vec![false; n]; n];
    for i in 0..n {
        for j in 0..i {
            let (x, y) = xy[i];
            let (xx, yy) = xy[j];
            if (x - xx) * (x - xx) + (y - yy) * (y - yy) <= d * d {
                g[i][j] = true;
                g[j][i] = true;
            }
        }
    }
    let mut que = VecDeque::new();
    let mut seen = vec![false; n];
    seen[0] = true;
    que.push_back(0);
    while let Some(u) = que.pop_front() {
        for v in 0..n {
            if g[u][v] && seen[v] == false {
                seen[v] = true;
                que.push_back(v);
            }
        }
    }
    for v in 0..n {
        if seen[v] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
