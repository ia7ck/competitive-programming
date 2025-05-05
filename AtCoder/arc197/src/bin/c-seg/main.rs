use proconio::input;
use segment_tree::SegmentTree;

fn main() {
    input! {
        q: usize,
        ab: [(usize, usize); q],
    };

    const M: usize = 3_000_000;
    let mut seg = SegmentTree::new(M, 0, |a, b| a + b);
    for i in 0..M {
        seg.update(i, |_| 1);
    }
    for (a, b) in ab {
        if a < M && seg[a] == 1 {
            for x in (a..M).step_by(a) {
                seg.update(x, |_| 0);
            }
        }
        let mut ok = 0;
        let mut ng = M + 1;
        while ng - ok > 1 {
            let mid = (ng + ok) / 2;
            if seg.fold(0..mid) <= b {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        println!("{}", ok);
    }
}
