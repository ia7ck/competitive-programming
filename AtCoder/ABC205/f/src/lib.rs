use std::borrow::Borrow;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct Edge {
    to: usize,
    cap: u64,
    rev: usize,
}

pub struct FordFulkerson {
    g: Vec<Vec<Edge>>,
}

impl FordFulkerson {
    pub fn new(n: usize) -> Self {
        Self { g: vec![vec![]; n] }
    }
    pub fn add_edge(&mut self, from: usize, to: usize, cap: u64) {
        let e1 = Edge {
            to,
            cap,
            rev: self.g[to].len(),
        };
        let from_size = if from == to {
            self.g[from].len() + 1
        } else {
            self.g[from].len()
        };
        let e2 = Edge {
            to: from,
            cap: 0,
            rev: from_size,
        };
        self.g[from].push(e1);
        self.g[to].push(e2);
        // assert_eq!(self.g[e1.to][e1.rev], e2);
        // assert_eq!(self.g[e2.to][e2.rev], e1);
    }
    fn bfs(&mut self, source: usize, sink: usize) -> Option<u64> {
        let n = self.g.len();
        let mut prev = vec![!0; n];

        let mut run = || {
            let mut flow = vec![0; n];
            let mut que = VecDeque::new();
            prev[source] = source;
            flow[source] = std::u64::MAX;
            que.push_back(source);
            while let Some(u) = que.pop_front() {
                if u == sink {
                    return Some(flow[u]);
                }
                for e in self.g[u].iter() {
                    if prev[e.to] < n || e.cap == 0 {
                        continue;
                    }
                    prev[e.to] = u;
                    assert_eq!(flow[e.to], 0);
                    flow[e.to] = flow[u].min(e.cap);
                    que.push_back(e.to);
                }
            }
            None
        };
        let f = run()?;
        let mut u = sink;
        while source != u {
            let e = self.g[u].iter_mut().find(|e| e.to == prev[u]).unwrap();
            e.cap += f;
            let &Edge { to, rev, .. } = e.borrow();
            self.g[to][rev].cap -= f;
            u = prev[u];
        }
        Some(f)
    }
    pub fn max_flow(&mut self, source: usize, sink: usize) -> u64 {
        let mut mf = 0;
        while let Some(f) = self.bfs(source, sink) {
            mf += f;
        }
        mf
    }
}
