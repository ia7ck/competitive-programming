use std::collections::HashMap;

use proconio::{input, marker::Usize1};

use crate::tree_diameter::tree_diameter;

fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1],
        c: [Usize1; n],
    };

    let mut g = vec![vec![]; n];
    for &(u, v) in &edges {
        g[u].push(v);
        g[v].push(u);
    }
    let edges = edges
        .into_iter()
        .map(|(u, v)| (u, v, 1))
        .collect::<Vec<_>>();

    let (diam, path) = tree_diameter(n, &edges);
    let diam = diam as usize;

    if diam % 2 == 1 {
        let (a, b) = (path[diam / 2], path[diam / 2 + 1]);
        let mut d_a = vec![usize::MAX; n];
        let mut d_b = vec![usize::MAX; n];
        d_a[a] = 0;
        d_b[b] = 0;
        dfs(a, b, &g, &mut d_a);
        dfs(b, a, &g, &mut d_b);

        let mut count_a = HashMap::<usize, usize>::new();
        let mut count_b = HashMap::<usize, usize>::new();
        for i in 0..n {
            if d_a[i] == diam / 2 {
                *count_a.entry(c[i]).or_insert(0) += 1;
            }
            if d_b[i] == diam / 2 {
                *count_b.entry(c[i]).or_insert(0) += 1;
            }
        }
        let mut ans = 0;
        for i in 0..n {
            let c_a = count_a.get(&i).unwrap_or(&0);
            let c_b = count_b.get(&i).unwrap_or(&0);
            ans += c_a * c_b;
        }
        println!("{}", ans);
    } else {
        let root = path[diam / 2];
        let mut count = HashMap::<usize, Vec<usize>>::new();
        for &i in &g[root] {
            let mut c_i = HashMap::<usize, usize>::new();
            dfs1(i, root, &g, &c, diam, 1, &mut c_i);
            for (k, v) in c_i {
                count.entry(k).or_insert(Vec::new()).push(v);
            }
        }
        let mut ans = 0;
        for (_, count) in count {
            // Σ_{a<b} count[a]*count[b]
            let mut sum = 0;
            for v in count {
                ans += sum * v;
                sum += v;
            }
        }
        println!("{}", ans);
    }
}

fn dfs(i: usize, p: usize, g: &Vec<Vec<usize>>, d: &mut Vec<usize>) {
    for &j in &g[i] {
        if j == p {
            continue;
        }
        d[j] = d[i] + 1;
        dfs(j, i, g, d);
    }
}

fn dfs1(
    i: usize,
    p: usize,
    g: &Vec<Vec<usize>>,
    c: &Vec<usize>,
    diam: usize,
    d: usize,
    count: &mut HashMap<usize, usize>,
) {
    if d == diam / 2 {
        *count.entry(c[i]).or_insert(0) += 1;
    }
    for &j in &g[i] {
        if j == p {
            continue;
        }
        dfs1(j, i, g, c, diam, d + 1, count);
    }
}

// Bundled
// https://github.com/ia7ck/rust-competitive-programming/blob/07c9c3fab02734d8dd4021491989d1817cdaad40/libs/tree_diameter/src/lib.rs
#[rustfmt::skip]
#[allow(unused)]
mod tree_diameter {
    use graph::is_tree;

    pub fn tree_diameter(n: usize, edges: &[(usize, usize, u64)]) -> (u64, Vec<usize>) {
        if n == 0 {
            return (0, Vec::new());
        }

        assert!(is_tree(
            n,
            &edges
                .iter()
                .copied()
                .map(|(u, v, _)| (u, v))
                .collect::<Vec<_>>()
        ));

        for &(_, _, c) in edges {
            assert!(c >= 1);
        }

        let mut graph = vec![vec![]; n];
        for &(u, v, c) in edges {
            graph[u].push((v, c));
            graph[v].push((u, c));
        }

        fn dfs(
            i: usize,
            p: usize,
            g: &[Vec<(usize, u64)>],
            dist: &mut Vec<u64>,
            parent: &mut Vec<usize>,
        ) {
            parent[i] = p;
            for &(j, c) in &g[i] {
                if j == p {
                    continue;
                }
                dist[j] = dist[i] + c;
                dfs(j, i, g, dist, parent);
            }
        }

        let mut dist = vec![0; n];
        let mut parent = vec![0; n];
        dfs(0, 0, &graph, &mut dist, &mut parent);

        let max_dist = dist.iter().max().copied().unwrap();
        let s = (0..n).position(|i| dist[i] == max_dist).unwrap();
        dist = vec![0; n];
        parent = vec![s; n];
        dfs(s, s, &graph, &mut dist, &mut parent);

        let diameter = dist.iter().max().copied().unwrap();
        let t = (0..n).position(|i| dist[i] == diameter).unwrap();

        let mut cur = t;
        let mut path = Vec::new();
        path.push(cur);
        while cur != parent[cur] {
            cur = parent[cur];
            path.push(cur);
        }
        (diameter, path)
    }

    mod graph {
        use std::mem;

        pub fn is_tree(n: usize, edges: &[(usize, usize)]) -> bool {
            for &(a, b) in edges {
                assert!(a < n);
                assert!(b < n);
                assert_ne!(a, b);
            }

            if n == 0 {
                return true;
            }

            edges.len() == n - 1 && connectivity(n, edges)
        }

        pub fn connectivity(n: usize, edges: &[(usize, usize)]) -> bool {
            fn dfs(i: usize, g: &[Vec<usize>], visited: &mut Vec<bool>) {
                for &j in &g[i] {
                    if visited[j] {
                        continue;
                    }
                    visited[j] = true;
                    dfs(j, g, visited);
                }
            }

            let mut g = vec![vec![]; n];
            for &(a, b) in edges {
                g[a].push(b);
                g[b].push(a);
            }
            let mut visited = vec![false; n];
            visited[0] = true;
            dfs(0, &g, &mut visited);
            visited.iter().filter(|&&f| f).count() == n
        }

        pub fn tree_drop_parent(
            n: usize,
            root: usize,
            edges: &[(usize, usize)],
        ) -> (Vec<Vec<usize>>, Vec<usize>) {
            debug_assert!(is_tree(n, edges));

            fn dfs(i: usize, p: usize, g: &Vec<Vec<usize>>, parent: &mut Vec<usize>) {
                parent[i] = p;
                for &j in &g[i] {
                    if j == p {
                        continue;
                    }
                    dfs(j, i, g, parent);
                }
            }

            let mut g = vec![vec![]; n];
            for &(a, b) in edges {
                g[a].push(b);
                g[b].push(a);
            }
            let mut parent = vec![usize::MAX; n];
            dfs(root, root, &g, &mut parent);

            for i in 0..n {
                g[i] = mem::take(&mut g[i])
                    .into_iter()
                    .filter(|&j| j != parent[i])
                    .collect();
            }

            (g, parent)
        }
    }
}
