use proconio::{input, marker::Usize1};
use segment_tree::SegmentTree;

fn main() {
    input! {
        n: usize,
        c: i64,
        m: usize,
        tp: [(Usize1, i64); m],
    };

    // dp[t[i]] = p[i] + max {
    //      - c * t[i] + max { dp[j] + c * j ; j < t[i] },
    //        c * t[i] + max { dp[j] - c * j ; t[i] <= j },
    // }

    let mut seg1 = SegmentTree::new(n, i64::MIN / 2, |a, b| *a.max(b));
    let mut seg2 = SegmentTree::new(n, i64::MIN / 2, |a, b| *a.max(b));
    seg1.update(0, 0);
    seg2.update(0, 0);
    let mut ans = 0;
    for (t, p) in tp {
        let x = p + (-c * t as i64 + seg1.fold(0..t)).max(c * t as i64 + seg2.fold(t..n));
        ans = ans.max(x);
        seg1.update(t, (x + c * t as i64).max(*seg1.get(t)));
        seg2.update(t, (x - c * t as i64).max(*seg2.get(t)));
    }
    println!("{}", ans);
}
