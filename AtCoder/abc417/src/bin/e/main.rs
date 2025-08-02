use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            x: Usize1,
            y: Usize1,
            edges: [(Usize1, Usize1); m],
        };

        solve(n, m, x, y, edges);
    }
}

fn solve(n: usize, _m: usize, x: usize, y: usize, edges: Vec<(usize, usize)>) {
    let mut g = vec![vec![]; n];
    for &(u, v) in &edges {
        g[u].push(v);
        g[v].push(u);
    }
    for i in 0..n {
        g[i].sort_unstable();
    }

    let mut t = 1;
    let mut seen = vec![0; n];
    let mut path = vec![x];
    loop {
        let u = path.last().copied().unwrap();
        let mut next = Option::<usize>::None;
        let path_set = path.iter().copied().collect::<HashSet<_>>();
        // g[u]: sorted
        for &v in &g[u] {
            f(v, &g, &path_set, t, &mut seen);
            let reach_y = seen[y] == t;
            t += 1;
            if reach_y {
                next = Some(v);
                break;
            }
        }
        let next = next.unwrap_or_else(|| panic!("!?"));
        path.push(next);
        if next == y {
            break;
        }
    }

    println!(
        "{}",
        path.iter()
            .map(|x| (x + 1).to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn f(i: usize, g: &Vec<Vec<usize>>, path: &HashSet<usize>, t: usize, seen: &mut Vec<usize>) {
    if seen[i] == t || path.contains(&i) {
        return;
    }
    seen[i] = t;
    for &j in &g[i] {
        f(j, g, path, t, seen);
    }
}
