use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1, u64); n - 1],
        d: [u64; n],
    };

    let edges = edges
        .into_iter()
        .map(|(u, v, w)| (u, v, E(w)))
        .collect::<Vec<_>>();

    let ans = rerooting(n, &edges, &S { d });
    for ans in ans {
        println!("{}", ans.1);
    }
}

#[derive(Clone)]
struct E(u64);

#[derive(Clone)]
struct V(usize, u64);

struct S {
    d: Vec<u64>,
}

impl RerootingOperator for S {
    type EdgeAttr = E;
    type Value = V;

    fn new(&self, i: usize) -> Self::Value {
        V(i, 0)
    }

    fn fold(&self, p: &Self::Value, ch: &Self::Value, e: &Self::EdgeAttr) -> Self::Value {
        V(p.0, p.1.max(ch.1 + e.0).max(self.d[ch.0] + e.0))
    }
}

trait RerootingOperator {
    type EdgeAttr: Clone;
    type Value: Clone;
    fn new(&self, i: usize) -> Self::Value;
    fn fold(&self, p: &Self::Value, ch: &Self::Value, e: &Self::EdgeAttr) -> Self::Value;
}

fn rerooting<R>(n: usize, edges: &[(usize, usize, R::EdgeAttr)], op: &R) -> Vec<R::Value>
where
    R: RerootingOperator,
{
    let mut g = vec![vec![]; n];
    for (u, v, e) in edges {
        g[*u].push((*v, e));
        g[*v].push((*u, e));
    }
    let mut pre_order = Vec::new();
    let mut stack = vec![(0, usize::MAX)];
    while let Some((i, p)) = stack.pop() {
        pre_order.push(i);
        g[i].retain(|&(j, _)| j != p);
        for (j, _) in &g[i] {
            stack.push((*j, i));
        }
    }

    let dp_sub = {
        let mut dp_sub = (0..n).map(|i| op.new(i)).collect::<Vec<_>>();
        for &i in pre_order.iter().rev() {
            for (j, e) in &g[i] {
                dp_sub[i] = op.fold(&dp_sub[i], &dp_sub[*j], e);
            }
        }
        dp_sub
    };

    fn apply<R>(
        acc: R::Value,
        children: &[(usize, &R::EdgeAttr)],
        op: &R,
        dp_sub: &Vec<R::Value>,
        dp: &mut Vec<R::Value>,
    ) where
        R: RerootingOperator,
    {
        if children.is_empty() {
            return;
        }
        if children.len() == 1 {
            let (j, e) = &children[0];
            // assert_eq!(dp[*j], op.new(*j));
            dp[*j] = op.fold(&dp[*j], &acc, e);
            return;
        }
        let (left, right) = children.split_at(children.len() / 2);
        let left_acc = left
            .iter()
            .fold(acc.clone(), |acc, (j, e)| op.fold(&acc, &dp_sub[*j], e));
        let right_acc = right
            .iter()
            .fold(acc, |acc, (j, e)| op.fold(&acc, &dp_sub[*j], e));
        apply(left_acc, right, op, dp_sub, dp);
        apply(right_acc, left, op, dp_sub, dp);
    }

    let mut dp = (0..n).map(|i| op.new(i)).collect::<Vec<_>>();
    for i in pre_order {
        apply(dp[i].clone(), &g[i], op, &dp_sub, &mut dp);
        
        for (j, e) in &g[i] {
            dp[i] = op.fold(&dp[i], &dp_sub[*j], e);
        }
    }

    dp
}
