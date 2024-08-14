use proconio::{input, marker::Usize1};

const P: usize = 1_000_000_000 + 7;

fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1],
    }

    let edges = edges
        .into_iter()
        .map(|(u, v)| (u, v, ()))
        .collect::<Vec<_>>();

    let mut fact = vec![1; n + 1];
    for i in 2..=n {
        fact[i] = fact[i - 1] * i % P;
    }
    let mut inv_fact = vec![1; n + 1];
    for i in 2..=n {
        inv_fact[i] = mpow(fact[i], P - 2);
    }
    for i in 1..=n {
        assert_eq!(fact[i] * inv_fact[i] % P, 1);
    }

    let ans = rerooting(n, &edges, S { fact, inv_fact });
    for ans in ans {
        println!("{}", ans.dp);
    }
}

fn mpow(a: usize, x: usize) -> usize {
    if x == 0 {
        1
    } else if x % 2 == 0 {
        mpow(a * a % P, x / 2)
    } else {
        a * mpow(a, x - 1) % P
    }
}

struct S {
    fact: Vec<usize>,
    inv_fact: Vec<usize>,
}

#[derive(Clone, Debug)]
struct V {
    s: usize,
    dp: usize,
}

impl RerootingOperator for S {
    type EdgeAttr = ();
    type Value = V;

    fn identity(&self) -> Self::Value {
        V { s: 0, dp: 1 }
    }

    fn fold(&self, _c_i: usize, c_v: &Self::Value, _e: &Self::EdgeAttr) -> Self::Value {
        V {
            s: c_v.s + 1,
            dp: c_v.dp,
        }
    }

    fn reduce(&self, a_v: &Self::Value, b_v: &Self::Value) -> Self::Value {
        let choose = self.fact[a_v.s + b_v.s] * self.inv_fact[a_v.s] % P * self.inv_fact[b_v.s] % P;
        V {
            s: a_v.s + b_v.s,
            dp: choose * a_v.dp % P * b_v.dp % P,
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
