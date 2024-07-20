use proconio::{input, marker::Usize1};
use segment_tree::SegmentTree;

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
    };

    let mut ans = vec![isize::MAX; n];

    // a[i] := p[i] + i
    // b[i] := p[i] - i
    let mut seg_a = SegmentTree::new(n, isize::MIN / 2, |a, b| *a.max(b));
    let mut seg_b = SegmentTree::new(n, isize::MAX / 2, |a, b| *a.min(b));
    for i in 0..n {
        let a = (p[i] + i) as isize;
        let b = p[i] as isize - i as isize;
        ans[i] = ans[i].min(a - seg_a.fold(0..p[i]));
        ans[i] = ans[i].min(seg_b.fold(p[i]..n) - b);
        seg_a.update(p[i], a);
        seg_b.update(p[i], b);
    }

    let mut seg_a = SegmentTree::new(n, isize::MAX / 2, |a, b| *a.min(b));
    let mut seg_b = SegmentTree::new(n, isize::MIN / 2, |a, b| *a.max(b));
    for i in (0..n).rev() {
        let a = (p[i] + i) as isize;
        let b = p[i] as isize - i as isize;
        ans[i] = ans[i].min(seg_a.fold(p[i]..n) - a);
        ans[i] = ans[i].min(b - seg_b.fold(0..p[i]));
        seg_a.update(p[i], a);
        seg_b.update(p[i], b);
    }

    for i in 0..n {
        assert!(ans[i] >= 0);
        print!("{}", ans[i]);
        if i + 1 < n {
            print!(" ")
        } else {
            print!("\n");
        }
    }
}
