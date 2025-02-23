use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1],
    };

    let edges = edges
        .into_iter()
        .map(|(u, v)| (u, v, E {}))
        .collect::<Vec<_>>();

    let vs = rerooting(n, &edges, &S {});
    let mut ans = 0;
    for top4 in vs {
        if top4.len() == 4 {
            // 次数4以上の頂点を根にしたときだけ考えれば十分
            let size = 1 + top4.iter().sum::<usize>();
            ans = ans.max(size);
        }
    }
    if ans == 0 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}

#[derive(Clone)]
struct E;

type V = Vec<usize>; // top4

struct S;

impl RerootingOperator for S {
    type EdgeAttr = E;
    type Value = V;

    fn new(&self, _i: usize) -> Self::Value {
        Vec::new()
    }

    fn fold(&self, p: &Self::Value, ch: &Self::Value, _e: &Self::EdgeAttr) -> Self::Value {
        assert!(p.len() <= 4);
        assert!(ch.len() <= 4);
        let new = if ch.len() >= 3 {
            // p と ch をつなげるので ch の寄与は top3 の和
            1 + ch.iter().take(3).sum::<usize>()
        } else {
            1
        };
        let mut top4 = p.clone();
        top4.push(new);
        top4.sort_unstable();
        top4.reverse();
        top4.truncate(4);
        top4
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
