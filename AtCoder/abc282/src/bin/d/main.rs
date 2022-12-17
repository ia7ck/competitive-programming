use proconio::{input, marker::Usize1};
use union_find::UnionFind;

fn dfs(i: usize, g: &Vec<Vec<usize>>, color: &mut Vec<isize>) -> bool {
    for &j in &g[i] {
        if color[j] == -1 {
            color[j] = if color[i] % 2 == 0 {
                color[i] + 1
            } else {
                color[i] - 1
            };
            if dfs(j, g, color) == false {
                return false;
            }
        } else if color[i] == color[j] {
            return false;
        }
    }
    true
}

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    };

    let mut g = vec![vec![]; n];
    let mut uf = UnionFind::new(n);
    for &(u, v) in &edges {
        g[u].push(v);
        g[v].push(u);
        uf.unite(u, v);
    }

    let mut color = vec![-1; n];
    let mut t = 0;
    for i in 0..n {
        if color[i] != -1 {
            continue;
        }
        color[i] = t;
        t += 2;
        if dfs(i, &g, &mut color) == false {
            println!("0");
            return;
        };
    }
    let mut freq = vec![0; n * 2 + 1];
    for i in 0..n {
        freq[color[i] as usize] += 1;
    }

    let mut ans = 0;
    for i in 0..n {
        let s = uf.get_size(i);
        if color[i] % 2 == 0 {
            ans += (n - s) + (freq[color[i] as usize + 1] - g[i].len());
        } else {
            ans += (n - s) + (freq[color[i] as usize - 1] - g[i].len());
        }
    }
    assert_eq!(ans % 2, 0);
    println!("{}", ans / 2);
}
