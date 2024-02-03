use proconio::input;
use segment_tree::SegmentTree;

fn main() {
    input! {
        n: usize,
        d: usize,
        a: [usize; n],
    };

    let m = a.iter().max().copied().unwrap();
    let mut seg = SegmentTree::new(m + 1, 0_usize, |x, y| *x.max(y));
    for a in a {
        let y = seg.fold(a.saturating_sub(d)..(a + d + 1).min(m + 1));
        seg.update(a, y + 1);
    }
    let ans = seg.fold(0..m + 1);
    println!("{}", ans);
}
