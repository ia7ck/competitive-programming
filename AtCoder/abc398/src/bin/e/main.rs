use std::collections::BTreeSet;

use proconio::{input_interactive, marker::Usize1};

fn main() {
    input_interactive! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1],
    };

    let mut g = vec![vec![]; n];
    for &(a, b) in &edges {
        g[a].push(b);
        g[b].push(a);
    }
    let mut depth = vec![0; n];
    dfs(0, n, &g, &mut depth);

    // 二部グラフ
    // depth[i] % 2 == 0 : left
    // depth[i] % 2 == 1 : right

    let edges = edges
        .into_iter()
        .map(|(a, b)| (a.min(b), a.max(b)))
        .collect::<BTreeSet<_>>();
    // a < b
    let mut valid = BTreeSet::new();
    for a in 0..n {
        for b in (a + 1)..n {
            if edges.contains(&(a, b)) {
                continue;
            }
            if depth[a] % 2 != depth[b] % 2 {
                valid.insert((a, b));
            }
        }
    }

    if valid.len() % 2 == 1 {
        println!("First");
    } else {
        println!("Second");
        input_interactive! {
            a: isize,
            b: isize,
        }
        if (a, b) == (-1, -1) {
            return;
        }
        let a = (a - 1) as usize;
        let b = (b - 1) as usize;
        assert!(valid.contains(&(a, b)));
        valid.remove(&(a, b));
    }

    while let Some((a, b)) = valid.pop_first() {
        println!("{} {}", a + 1, b + 1);
        input_interactive! {
            a: isize,
            b: isize,
        }
        if (a, b) == (-1, -1) {
            return;
        }
        let a = (a - 1) as usize;
        let b = (b - 1) as usize;
        assert!(valid.contains(&(a, b)));
        valid.remove(&(a, b));
    }
}

fn dfs(i: usize, p: usize, g: &Vec<Vec<usize>>, depth: &mut Vec<usize>) {
    for &j in &g[i] {
        if j == p {
            continue;
        }
        depth[j] = depth[i] + 1;
        dfs(j, i, g, depth);
    }
}
