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
            edges: [(Usize1, Usize1); m],
        };

        solve(n, m, edges);
    }
}

fn solve(n: usize, _m: usize, edges: Vec<(usize, usize)>) {
    if n <= 2 {
        println!("-1");
        return;
    }

    let mut g = vec![vec![]; n];
    let mut edge_set = HashSet::new();
    for (a, b) in edges {
        g[a].push(b);
        g[b].push(a);
        edge_set.insert((a, b));
        edge_set.insert((b, a));
    }
    let mut color = vec![None; n];
    color[0] = Some(true);
    let Some(ans) = dfs(0, usize::MAX, &g, &mut color) else {
        println!("-1");
        return;
    };

    assert!(ans.closed);

    let ans = ans.vs;

    assert!(ans.len() >= 3);
    assert_eq!(ans.len() % 2, 1);
    let ans_set = ans.iter().copied().collect::<HashSet<_>>();
    assert_eq!(ans.len(), ans_set.len());
    for i in 0..ans.len() {
        assert!(edge_set.contains(&(ans[i], ans[(i + 1) % ans.len()])));
    }

    println!("{}", ans.len());
    println!(
        "{}",
        ans.iter()
            .map(|&x| (x + 1).to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

#[derive(Debug)]
struct Cycle {
    vs: Vec<usize>,
    end: usize,
    closed: bool,
}

fn dfs(
    cur: usize,
    par: usize,
    g: &Vec<Vec<usize>>,
    color: &mut Vec<Option<bool>>,
) -> Option<Cycle> {
    let c = color[cur].unwrap();

    for &next in &g[cur] {
        if next == par {
            continue;
        }

        match color[next] {
            Some(nc) => {
                if c == nc {
                    return Some(Cycle {
                        vs: vec![cur],
                        end: next,
                        closed: false,
                    });
                }
            }
            None => {
                color[next] = Some(!c);
                if let Some(mut cycle) = dfs(next, cur, g, color) {
                    if cycle.closed {
                        return Some(cycle);
                    }
                    cycle.vs.push(cur);
                    if cur == cycle.end {
                        cycle.closed = true;
                    }
                    return Some(cycle);
                }
            }
        }
    }

    None
}
