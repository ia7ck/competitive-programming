use next_permutation::NextPermutation;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1, u64); m],
        q: usize,
    };

    let mut d = vec![vec![u64::MAX / 2; n]; n];
    for i in 0..n {
        d[i][i] = 0;
    }
    for &(u, v, w) in &edges {
        d[u][v] = d[u][v].min(w);
        d[v][u] = d[v][u].min(w);
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                d[i][j] = d[i][j].min(d[i][k] + d[k][j]);
            }
        }
    }

    for _ in 0..q {
        input! {
            k: usize,
            mut b: [Usize1; k],
        };

        let mut ans = u64::MAX;
        loop {
            for rev in 0..(1 << b.len()) {
                let mut dist = 0;
                let mut x = 0;
                for (i, &b) in b.iter().enumerate() {
                    let (u, v, w) = edges[b];
                    let (u, v) = if rev >> i & 1 == 1 { (v, u) } else { (u, v) };
                    dist += d[x][u];
                    dist += w;
                    x = v;
                }
                dist += d[x][n - 1];
                ans = ans.min(dist);
            }

            if !b.next_permutation() {
                break;
            }
        }
        println!("{}", ans);
    }
}
