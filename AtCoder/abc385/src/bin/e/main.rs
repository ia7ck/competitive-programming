use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1],
    };

    let mut g = vec![vec![]; n];
    let mut deg = vec![0; n];
    for &(u, v) in &edges {
        g[u].push(v);
        g[v].push(u);
        deg[u] += 1;
        deg[v] += 1;
    }

    let mut ans = n;
    for r in 0..n {
        let mut children = g[r].iter().copied().collect::<Vec<_>>();
        children.sort_by_key(|&v| deg[v]);
        children.reverse();
        for x in 1..=deg[r] {
            let y = deg[children[x - 1]] - 1;
            if y >= 1 {
                ans = ans.min(n - (x + x * y + 1));
            }
        }
    }
    println!("{}", ans);
}
