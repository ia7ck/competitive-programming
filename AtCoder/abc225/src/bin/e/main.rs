use input_i_scanner::{scan_with, InputIScanner};
use segment_tree::SegmentTree;

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let n = scan_with!(_i_i, usize);
    #[derive(Copy, Clone, Debug)]
    struct Fu((u64, u64));
    impl Fu {
        fn left(&self) -> (u64, u64) {
            let (x, y) = self.0;
            (x - 1, y)
        }
        fn bottom(&self) -> (u64, u64) {
            let (x, y) = self.0;
            (x, y - 1)
        }
    }
    let mut points = Vec::new();
    for _ in 0..n {
        let (x, y) = scan_with!(_i_i, (u64, u64));
        points.push(Fu((x, y)));
    }
    points.sort_by(|p, q| {
        let (xp, yp) = p.left();
        let (xq, yq) = q.left();
        // yp/xp <=> yq/xq
        let ord = (yp * xq).cmp(&(yq * xp)).then(
            // ??
            // sqrt((xp * xp) + (yp * yp)) <=> sqrt(...)
            ((xp * xp) + (yp * yp)).cmp(&((xq * xq) + (yq * yq))),
        );
        ord
    });
    let intersect = |p: &Fu, q: &Fu| -> bool {
        let (x, y) = p.bottom();
        let (xx, yy) = q.left();
        // y/x < yy/xx
        y * xx < yy * x
    };
    let mut seg = SegmentTree::new(n, 0usize, |&a, &b| a.max(b));
    seg.update(0, 1);
    for i in 1..n {
        let p = points[i];
        if intersect(&p, &points[0]) {
            seg.update(i, 1);
            continue;
        }
        let mut ok = 0;
        let mut ng = i;
        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            if intersect(&p, &points[mid]) {
                ng = mid;
            } else {
                ok = mid;
            }
        }
        let mx = seg.fold(0..(ok + 1));
        seg.update(i, mx + 1);
    }
    println!("{}", seg.fold(0..n));
}
