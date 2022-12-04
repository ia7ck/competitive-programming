use proconio::input;
use segment_tree::SegmentTree;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
    };

    let mut cum_sum = vec![0; n + 1];
    for i in 0..n {
        cum_sum[i + 1] = cum_sum[i] + a[i];
    }

    let mut dp_0 = vec![0; n + 1];
    let mut dp_1 = vec![0; n + 1];
    let mut seg = SegmentTree::new(n + 1, std::i64::MIN / 2, |a, b| *a.max(b));
    for i in 0..=n {
        seg.update(i, 0);
    }
    for i in 0..n {
        dp_0[i + 1] = dp_0[i].max(dp_1[i]);
        
        // for j in i.saturating_sub(k - 2)..=i {
        //     dp_1[i + 1] = dp_1[i + 1].max(dp_0[j] + (cum_sum[i + 1] - cum_sum[j]));
        // }

        let v = seg.fold(i.saturating_sub(k - 2)..(i + 1));
        dp_1[i + 1] = v + cum_sum[i + 1];
        seg.update(i + 1, dp_0[i + 1] - cum_sum[i + 1]);
    }

    let ans = dp_0[n].max(dp_1[n]);
    println!("{}", ans);
}
