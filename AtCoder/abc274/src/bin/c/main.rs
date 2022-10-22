use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut g = vec![vec![]; n * 2 + 2];
    for i in 0..n {
        g[a[i]].push((i + 1) * 2);
        g[a[i]].push((i + 1) * 2 + 1);
    }

    let mut d = vec![0; n * 2 + 2];
    let mut que = VecDeque::new();
    d[1] = 0;
    que.push_back(1);
    while let Some(u) = que.pop_front() {
        for &v in &g[u] {
            d[v] = d[u] + 1;
            que.push_back(v);
        }
    }
    for k in 1..=(n * 2 + 1) {
        println!("{}", d[k]);
    }
}
