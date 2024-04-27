use proconio::input;
use segment_tree::SegmentTree;
use zarts::SortedSeq;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    };

    let b = SortedSeq::from_iter(a.iter().copied());
    let mut count = SegmentTree::new(b.size(), 0, |x, y| x + y);
    let mut value = SegmentTree::new(b.size(), 0, |x, y| x + y);
    for &x in &a {
        let i = b.ord(&x);
        count.update(i, count.get(i) + 1);
        value.update(i, value.get(i) + x);
    }

    let mut ans = 0;
    for &x in &a {
        let i = b.ord(&x);
        let c = count.fold((i + 1)..b.size());
        let v = value.fold((i + 1)..b.size());
        assert!(v >= x * c);
        ans += v - x * c;

        count.update(i, count.get(i) - 1);
        value.update(i, value.get(i) - x);
    }
    println!("{}", ans);
}
