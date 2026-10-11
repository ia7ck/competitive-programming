use std::collections::HashMap;

use proconio::{input, marker::Usize1};

use crate::graph::tree_drop_parent;

fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1],
    };

    let (children, _parent) = tree_drop_parent(n, 0, &edges);
    let edges = edges
        .into_iter()
        .enumerate()
        .map(|(i, (a, b))| ((a, b), (i + 1) as isize))
        .collect::<HashMap<_, _>>();
    let mut size = vec![0; n];

    dfs(0, &mut size, &children);

    let mut x = vec![0; n];
    solve_set(0, &mut x, &size, &children, &edges);

    // eprintln!("{x:?}");

    solve_cumsum(0, &mut x, &children);

    println!(
        "{}",
        x.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn dfs(i: usize, size: &mut Vec<usize>, children: &Vec<Vec<usize>>) {
    for &j in &children[i] {
        dfs(j, size, children);

        size[i] += size[j];
    }

    size[i] += 1;
}

fn solve_set(
    i: usize,
    x: &mut Vec<isize>,
    size: &Vec<usize>,
    children: &Vec<Vec<usize>>,
    edges: &HashMap<(usize, usize), isize>,
) {
    for &j in &children[i] {
        solve_set(j, x, size, children, edges);

        let y = match (edges.get(&(i, j)), edges.get(&(j, i))) {
            (Some(&y), None) => y,
            (None, Some(&y)) => y,
            _ => unreachable!(),
        };

        let sub = size[j];
        // n - size[j]
        let par = size.len() - size[j];
        if sub < par {
            x[0] += y;
            x[j] -= y;
        } else {
            x[j] += y;
        }
    }
}

fn solve_cumsum(i: usize, x: &mut Vec<isize>, children: &Vec<Vec<usize>>) {
    for &j in &children[i] {
        x[j] += x[i];

        solve_cumsum(j, x, children);
    }
}

// Bundled
#[rustfmt::skip]
#[allow(unused)]
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
