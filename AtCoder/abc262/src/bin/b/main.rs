use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize); m],
    };

    let mut g = vec![vec![false; n]; n];
    for (u, v) in edges {
        g[u - 1][v - 1] = true;
        g[v - 1][u - 1] = true;
    }

    let mut ans = 0;
    for a in 0..n {
        for b in (a + 1)..n {
            for c in (b + 1)..n {
                if g[a][b] && g[b][c] && g[c][a] {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
