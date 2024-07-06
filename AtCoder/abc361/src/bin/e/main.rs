use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1, u64); n - 1],
    };

    let edges = edges
        .into_iter()
        .map(|(u, v, w)| (u, v, E(w)))
        .collect::<Vec<_>>();
    let dp = rerooting(n, &edges, S);
    let ans = dp.iter().map(|v| v.tsp - v.dist).min().unwrap();
    println!("{}", ans);
}

struct S;

#[derive(Clone, Debug)]
struct E(u64);

#[derive(Clone, Debug)]
struct V {
    tsp: u64,
    dist: u64,
}

impl RerootingOperator for S {
    type EdgeAttr = E;
    type Value = V;

    fn identity(&self) -> Self::Value {
        V { tsp: 0, dist: 0 }
    }

    fn fold(&self, _: usize, c_v: &Self::Value, e: &Self::EdgeAttr) -> Self::Value {
        V {
            tsp: c_v.tsp + e.0 * 2,
            dist: c_v.dist + e.0,
        }
    }

    fn reduce(&self, a_v: &Self::Value, b_v: &Self::Value) -> Self::Value {
        V {
            tsp: a_v.tsp + b_v.tsp,
            dist: a_v.dist.max(b_v.dist),
        }
    }
}

trait RerootingOperator {
    type EdgeAttr: Clone;
    type Value: Clone;
    // reduce(a, identity()) == a
    fn identity(&self) -> Self::Value;
    fn fold(&self, c_i: usize, c_v: &Self::Value, e: &Self::EdgeAttr) -> Self::Value;
    fn reduce(&self, a_v: &Self::Value, b_v: &Self::Value) -> Self::Value;
}

fn rerooting<R>(n: usize, edges: &[(usize, usize, R::EdgeAttr)], op: R) -> Vec<R::Value>
where
    R: RerootingOperator,
{
    let mut g = vec![vec![]; n];
    for (u, v, e) in edges {
        g[*u].push((*v, e.clone()));
        g[*v].push((*u, e.clone()));
    }
    let mut order = Vec::new();
    let mut parent = vec![None; n];
    let mut stack = vec![(0, usize::MAX)];
    while let Some((i, p)) = stack.pop() {
        order.push(i);
        g[i].retain(|&(j, _)| j != p);
        for (j, e) in &g[i] {
            parent[*j] = Some((i, e.clone()));
            stack.push((*j, i));
        }
    }

    let mut dp = vec![op.identity(); n];
    for &i in order.iter().rev() {
        for (j, e) in &g[i] {
            dp[i] = op.reduce(&dp[i], &op.fold(*j, &dp[*j], e));
        }
    }

    let mut dp_p = vec![op.identity(); n];
    for &i in &order {
        if let Some((p, e)) = &parent[i] {
            dp_p[i] = op.fold(*p, &dp_p[i], e);
        }
        let mut acc = dp_p[i].clone();
        for (j, e) in &g[i] {
            dp_p[*j] = acc.clone();
            acc = op.reduce(&acc, &op.fold(*j, &dp[*j], e));
        }
        let mut acc = op.identity();
        for (j, e) in g[i].iter().rev() {
            dp_p[*j] = op.reduce(&dp_p[*j], &acc);
            acc = op.reduce(&op.fold(*j, &dp[*j], e), &acc);
        }
    }

    dp.into_iter()
        .zip(dp_p.into_iter())
        .map(|(dp, dp_p)| op.reduce(&dp, &dp_p))
        .collect()
}
