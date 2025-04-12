use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n1: usize,
        edges1: [(Usize1, Usize1); n1 - 1],
        n2: usize,
        edges2: [(Usize1, Usize1); n2 - 1],
    };

    let edges1 = edges1.iter().map(|&(u, v)| (u, v, E)).collect::<Vec<_>>();
    let d1 = rerooting(n1, &edges1, &S);
    let d1 = d1.iter().map(|v| v.top2[0]).collect::<Vec<_>>();
    let diam1 = d1.iter().max().copied().unwrap();
    let edges2 = edges2.iter().map(|&(u, v)| (u, v, E)).collect::<Vec<_>>();
    let d2 = rerooting(n2, &edges2, &S);
    let d2 = d2.iter().map(|v| v.top2[0]).collect::<Vec<_>>();
    let diam2 = d2.iter().max().copied().unwrap();

    let mut d2 = d2;
    d2.sort_unstable();
    let mut cum_sum = vec![0; n2 + 1];
    for i in 0..n2 {
        cum_sum[i + 1] = cum_sum[i] + d2[i];
    }

    let mut ans = 0_usize;
    for i in 0..n1 {
        let p = d2.partition_point(|&d2| d1[i] + 1 + d2 < diam1.max(diam2));
        let s1 = diam1.max(diam2) * p;
        let s2 = (d1[i] + 1) * (n2 - p) + (cum_sum[n2] - cum_sum[p]);
        ans += s1 + s2;
    }

    println!("{}", ans);
}

#[derive(Clone)]
struct E;

#[derive(Clone, Debug)]
struct V {
    top2: Vec<usize>,
}

impl V {
    fn new(top2: Vec<usize>) -> Self {
        Self { top2 }
    }
}

struct S;

impl RerootingOperator for S {
    type EdgeAttr = E;
    type Value = V;

    fn new(&self, _i: usize) -> Self::Value {
        V::new(vec![0, 0])
    }

    fn fold(&self, p: &Self::Value, ch: &Self::Value, _e: &Self::EdgeAttr) -> Self::Value {
        let mut top2 = Vec::with_capacity(p.top2.len() + ch.top2.len());
        top2.extend(&p.top2);
        for &ch in &ch.top2 {
            top2.push(ch + 1);
        }
        top2.sort_unstable();
        top2.reverse();
        top2.truncate(2);
        V::new(top2)
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
