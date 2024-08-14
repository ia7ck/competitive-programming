use proconio::{input, marker::Usize1};
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: u64,
        edges: [(Usize1, Usize1); n - 1],
    }

    let edges = edges
        .into_iter()
        .map(|(u, v)| (u, v, ()))
        .collect::<Vec<_>>();

    let rerooting = Rerooting::new(n, &edges, S { m });
    let ans = rerooting.solve();
    for ans in ans {
        println!("{}", ans);
    }
}

struct S {
    m: u64,
}

impl RerootingOperator for S {
    type EdgeAttr = ();
    type Value = u64;

    fn init(&self, _i: usize) -> Self::Value {
        1
    }

    fn fold(&self, _c_i: usize, c_v: &Self::Value, _e: &Self::EdgeAttr) -> Self::Value {
        (c_v + 1) % self.m
    }

    fn reduce(&self, a_v: &Self::Value, b_v: &Self::Value) -> Self::Value {
        a_v * b_v % self.m
    }
}

trait RerootingOperator {
    type EdgeAttr: Clone;
    type Value: Clone;
    // reduce(a, init()) == a
    fn init(&self, i: usize) -> Self::Value;
    fn fold(&self, c_i: usize, c_v: &Self::Value, e: &Self::EdgeAttr) -> Self::Value;
    // commutative
    fn reduce(&self, a_v: &Self::Value, b_v: &Self::Value) -> Self::Value;
}

struct Rerooting<R>
where
    R: RerootingOperator,
{
    n: usize,
    adj: Vec<Vec<(usize, R::EdgeAttr)>>,
    op: R,
}

impl<R> Rerooting<R>
where
    R: RerootingOperator,
{
    fn new(n: usize, edges: &[(usize, usize, R::EdgeAttr)], op: R) -> Self {
        let mut adj = vec![vec![]; n];
        for (u, v, e) in edges {
            adj[*u].push((*v, e.clone()));
            adj[*v].push((*u, e.clone()));
        }
        Self { n, adj, op }
    }

    fn solve(&self) -> Vec<R::Value> {
        let mut dp = (0..self.n).map(|v| self.op.init(v)).collect::<Vec<_>>();
        self.dfs(0, usize::MAX, &mut dp);

        let mut dp_root = vec![None; self.n];
        let mut que = VecDeque::new();
        que.push_back((0, usize::MAX, self.op.init(0)));
        while let Some((v, p, p_acc)) = que.pop_front() {
            let mut values = vec![p_acc];
            for (x, e) in &self.adj[v] {
                if *x != p {
                    values.push(self.op.fold(*x, &dp[*x], e));
                }
            }
            let n = values.len();
            let mut left_acc = values.clone();
            for i in 1..n {
                left_acc[i] = self.op.reduce(&left_acc[i], &left_acc[i - 1]);
            }
            dp_root[v] = Some(left_acc[n - 1].clone());
            let mut right_acc = values.clone();
            for i in (0..(n - 1)).rev() {
                right_acc[i] = self.op.reduce(&right_acc[i], &right_acc[i + 1]);
            }
            let mut index = 0;
            for (x, e) in &self.adj[v] {
                if *x != p {
                    let acc = if index + 2 < n {
                        self.op.reduce(&left_acc[index], &right_acc[index + 2])
                    } else {
                        left_acc[index].clone()
                    };
                    que.push_back((*x, v, self.op.fold(v, &acc, e)));
                    index += 1;
                }
            }
        }
        let dp_root = dp_root.into_iter().flatten().collect::<Vec<_>>();
        assert_eq!(dp_root.len(), self.n);
        dp_root
    }

    fn dfs(&self, v: usize, p: usize, dp: &mut Vec<R::Value>) {
        for (x, e) in &self.adj[v] {
            if *x == p {
                continue;
            }
            self.dfs(*x, v, dp);
            let child = self.op.fold(*x, &dp[*x], e);
            dp[v] = self.op.reduce(&dp[v], &child);
        }
    }
}
