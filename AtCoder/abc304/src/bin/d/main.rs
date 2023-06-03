use proconio::input;
use segment_tree::SegmentTree;

fn main() {
    input! {
        w: u32,
        _h: u32,
        n: usize,
        mut pq: [(u32, u32); n],
        n_a: usize,
        mut a: [u32; n_a],
        n_b: usize,
        b: [u32; n_b],
    };

    a.push(w);
    pq.sort();
    let mut xy = Vec::new();
    let mut i = 0;
    let mut seg_min = SegmentTree::new(n_b + 1, std::usize::MAX / 2, |a, b| *a.min(b));
    for i in 0..=n_b {
        seg_min.update(i, 0);
    }
    let mut seg_max = SegmentTree::new(n_b + 1, 0_usize, |a, b| *a.max(b));
    let mut ans_min = std::usize::MAX / 2;
    let mut ans_max = 0;
    for a in a {
        while i < n && pq[i].0 < a {
            xy.push(pq[i]);
            i += 1;
        }
        assert!(i == n || pq[i].0 > a);
        for &(_, y) in &xy {
            let k = b.binary_search(&y).unwrap_err();
            seg_min.update(k, seg_min.get(k) + 1);
            seg_max.update(k, seg_max.get(k) + 1);
        }
        ans_min = ans_min.min(seg_min.fold(0..(n_b + 1)));
        ans_max = ans_max.max(seg_max.fold(0..(n_b + 1)));
        for (_, y) in xy.drain(..) {
            let k = b.binary_search(&y).unwrap_err();
            seg_min.update(k, seg_min.get(k) - 1);
            seg_max.update(k, seg_max.get(k) - 1);
        }
    }

    println!("{} {}", ans_min, ans_max);
}
