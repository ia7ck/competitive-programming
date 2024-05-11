use proconio::input;
use segment_tree::SegmentTree;
use zarts::SortedSeq;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    };

    const P: u64 = 100_000_000;

    let mut cum_sum = vec![0; n + 1];
    for i in 0..n {
        cum_sum[i + 1] = cum_sum[i] + a[i];
    }

    let mut ans = 0;
    for i in 0..(n - 1) {
        ans += (n - i - 1) as u64 *a[i] + (cum_sum[n] - cum_sum[i + 1]);
    }

    let seq = {
        let mut values = Vec::new();
        for &x in &a {
            values.push(x);
            values.push(P - x);
        }
        SortedSeq::from_iter(values)
    };
    let mut seg = SegmentTree::new(seq.size(), 0, |x, y| x + y);
    let mut sub = 0;
    for i in (0..n).rev() {
        sub += seg.fold(seq.ord(&(P - a[i]))..seq.size());
        let pos = seq.ord(&a[i]);
        seg.update(pos, seg.get(pos) + 1);
    }
    assert!(ans >= sub * P);
    println!("{}", ans - sub * P);
}
