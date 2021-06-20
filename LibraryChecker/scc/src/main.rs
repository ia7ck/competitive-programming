use procon_reader::ProconReader;
use std::collections::VecDeque;
use std::ops::Index;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let m: usize = rd.get();
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        let a: usize = rd.get();
        let b: usize = rd.get();
        g[a].push(b);
    }

    let scc = StronglyConnectedComponents::new(&g);
    let h = scc.graph();
    let mut members = vec![vec![]; h.len()];
    for u in 0..n {
        members[scc[u]].push(u);
    }
    println!("{}", h.len());
    for i in 0..members.len() {
        print!("{}", members[i].len());
        for v in &members[i] {
            print!(" {}", v);
        }
        println!();
    }
}

pub struct StronglyConnectedComponents {
    component: Vec<usize>,
    graph: Vec<Vec<usize>>,
}

impl StronglyConnectedComponents {
    pub fn new(g: &[Vec<usize>]) -> Self {
        let n = g.len();
        let mut seen = vec![false; n];
        let mut order = Vec::new();
        let mut order_pushed = vec![false; n];

        for v in 0..n {
            if seen[v] {
                continue;
            }
            let mut dfs = |start: usize| {
                let mut stack = Vec::new();
                stack.push(start);
                while let Some(u) = stack.pop() {
                    seen[u] = true;
                    stack.push(u);
                    for &v in &g[u] {
                        if !seen[v] {
                            stack.push(v);
                        }
                    }
                    // if all children are visited
                    if stack.last() == Some(&u) {
                        stack.pop();
                        if !order_pushed[u] {
                            order_pushed[u] = true;
                            order.push(u);
                        }
                    }
                }
            };
            dfs(v);
        }

        let mut rg = vec![vec![]; n];
        for u in 0..n {
            for &v in &g[u] {
                rg[v].push(u);
            }
        }

        let mut seen = vec![false; n];
        let mut component = vec![0; n];
        let mut new_index = 0;
        for u in order.into_iter().rev() {
            if seen[u] {
                continue;
            }
            let mut dfs = |start: usize| {
                let mut stack = Vec::new();
                stack.push(start);
                while let Some(u) = stack.pop() {
                    seen[u] = true;
                    component[u] = new_index;
                    for &v in &rg[u] {
                        if !seen[v] {
                            stack.push(v);
                        }
                    }
                }
            };
            dfs(u);
            new_index += 1;
        }

        let mut h = vec![vec![]; new_index];
        for u in 0..n {
            for &v in &g[u] {
                let uu = component[u];
                let vv = component[v];
                if uu != vv {
                    h[uu].push(vv);
                }
            }
        }

        Self {
            component,
            graph: h,
        }
    }

    pub fn graph(&self) -> &[Vec<usize>] {
        &self.graph
    }
}

impl Index<usize> for StronglyConnectedComponents {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        self.component.index(index)
    }
}

pub fn topological_sort(g: &[Vec<usize>]) -> Vec<usize> {
    let n = g.len();
    let mut in_deg = vec![0; n];
    for u in 0..n {
        for &v in &g[u] {
            in_deg[v] += 1;
        }
    }
    let mut order = Vec::new();
    let mut que = VecDeque::new();
    for u in 0..n {
        if in_deg[u] == 0 {
            order.push(u);
            que.push_back(u);
        }
    }
    while let Some(u) = que.pop_front() {
        for &v in &g[u] {
            in_deg[v] -= 1;
            if in_deg[v] == 0 {
                order.push(v);
                que.push_back(v);
            }
        }
    }
    order
}
