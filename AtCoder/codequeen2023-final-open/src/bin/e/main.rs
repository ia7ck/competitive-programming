use std::{cmp::Reverse, collections::BTreeSet};

use proconio::input;

use segment_tree::SegmentTree;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };

    const INF: i64 = std::i64::MAX / 2;
    // dp[i] := max(dp[i - 1], max(dp[j - 1] + a[i] - a[j]), max(dp[j - 1] + a[j] - a[i]))
    let mut dp = vec![-INF; n];
    dp[0] = 0;
    // seg1[j] := dp[j - 1] - a[j]
    let mut seg1 = SegmentTree::new(n, -INF, |x, y| *x.max(y));
    seg1.update(0, -a[0]);
    // seg1[j] := dp[j - 1] + a[j]
    let mut seg2 = SegmentTree::new(n, -INF, |x, y| *x.max(y));
    seg2.update(0, a[0]);
    let mut set1 = BTreeSet::new();
    let mut set2 = BTreeSet::new();
    for i in 1..n {
        dp[i] = dp[i].max(dp[i - 1]);

        let j = set1
            .range((a[i], Reverse(0))..)
            .next()
            .copied()
            .map(|(_, Reverse(j))| j + 1)
            .unwrap_or(0);
        dp[i] = dp[i].max(seg1.fold(j..i) + a[i]);

        let j = set2
            .range(..=(a[i], n))
            .last()
            .copied()
            .map(|(_, j)| j + 1)
            .unwrap_or(0);
        dp[i] = dp[i].max(seg2.fold(j..i) - a[i]);

        seg1.update(i, if i == 0 { -a[i] } else { dp[i - 1] - a[i] });
        seg2.update(i, if i == 0 { a[i] } else { dp[i - 1] + a[i] });
        set1.insert((a[i], Reverse(i)));
        set2.insert((a[i], i));
    }

    println!("{}", dp[n - 1]);
}
