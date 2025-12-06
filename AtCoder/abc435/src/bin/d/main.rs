use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

use crate::strongly_connected_components::strongly_connected_components;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
        q: usize,
        queries: [(u8, Usize1); q],
    };

    let cmp = strongly_connected_components(n, &edges);
    let mut cmp_index = vec![0; n];
    for i in 0..cmp.len() {
        for &v in &cmp[i] {
            cmp_index[v] = i;
        }
    }

    // DAG
    let mut rev_g = vec![vec![]; cmp.len()];
    for &(x, y) in &edges {
        let x = cmp_index[x];
        let y = cmp_index[y];
        if x == y {
            continue;
        }
        rev_g[y].push(x);
    }

    let mut reachable = vec![false; cmp_index.len()];
    for (op, v) in queries {
        let v = cmp_index[v];
        if op == 1 {
            if reachable[v] {
                continue;
            }
            let mut que = VecDeque::new();
            reachable[v] = true;
            que.push_back(v);
            while let Some(v) = que.pop_front() {
                for &u in &rev_g[v] {
                    if reachable[u] == false {
                        reachable[u] = true;
                        que.push_back(u);
                    }
                }
            }
        } else {
            if reachable[v] {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}

// Bundled
#[rustfmt::skip]
#[allow(unused)]
mod strongly_connected_components {
    pub fn strongly_connected_components(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
        let mut graph = vec![vec![]; n];
        for &(u, v) in edges {
            graph[u].push(v);
        }

        let mut seen = vec![false; n];
        let mut order = Vec::new();
        let mut order_pushed = vec![false; n];
        for v in 0..n {
            if seen[v] {
                continue;
            }
            let mut stack = Vec::new();
            stack.push(v);
            while let Some(x) = stack.pop() {
                seen[x] = true;
                stack.push(x);
                let mut pushed = false;
                for &y in &graph[x] {
                    if !seen[y] {
                        stack.push(y);
                        pushed = true;
                    }
                }
                if !pushed {
                    debug_assert_eq!(stack.last(), Some(&x));
                    stack.pop();
                    if !order_pushed[x] {
                        order_pushed[x] = true;
                        order.push(x);
                    }
                }
            }
        }
        assert_eq!(order.len(), n);

        let mut rev_graph = vec![vec![]; n];
        #[allow(clippy::needless_range_loop)]
        for u in 0..n {
            for &v in &graph[u] {
                rev_graph[v].push(u);
            }
        }

        let mut seen = vec![false; n];
        let mut component_id = vec![0; n];
        let mut id = 0;
        for &v in order.iter().rev() {
            if seen[v] {
                continue;
            }
            let mut stack = Vec::new();
            stack.push(v);
            while let Some(x) = stack.pop() {
                seen[x] = true;
                component_id[x] = id;
                for &y in &rev_graph[x] {
                    if !seen[y] {
                        stack.push(y);
                    }
                }
            }
            id += 1;
        }

        let mut components = vec![vec![]; id];
        for v in 0..n {
            components[component_id[v]].push(v);
        }
        components
    }
}
