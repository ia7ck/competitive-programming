use proconio::{input, marker::Usize1};
use segment_tree::SegmentTree;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u64; n],
    };

    let mut seg = SegmentTree::new(
        n,
        (0, 0, 0, 0),
        |&(m1, c1, mm1, cc1), &(m2, c2, mm2, cc2)| {
            if m1 > m2 {
                if mm1 > m2 {
                    (m1, c1, mm1, cc1)
                } else if mm1 < m2 {
                    (m1, c1, m2, c2)
                } else {
                    (m1, c1, mm1, cc1 + c2)
                }
            } else if m1 < m2 {
                if m1 > mm2 {
                    (m2, c2, m1, c1)
                } else if m1 < mm2 {
                    (m2, c2, mm2, cc2)
                } else {
                    (m2, c2, m1, c1 + cc2)
                }
            } else {
                if mm1 > mm2 {
                    (m1, c1 + c2, mm1, cc1)
                } else if mm1 < mm2 {
                    (m1, c1 + c2, mm2, cc2)
                } else {
                    (m1, c1 + c2, mm1, cc1 + cc2)
                }
            }
        },
    );

    for i in 0..n {
        seg.update(i, (a[i], 1, 0, 0));
    }

    for _ in 0..q {
        input! {
            op: u8,
        };

        if op == 1 {
            input! {
                p: Usize1,
                x: u64,
            };
            seg.update(p, (x, 1, 0, 0));
        } else {
            input! {
                l: Usize1,
                r: Usize1,
            };
            let (_, _, _, ans) = seg.fold(l..(r + 1));
            println!("{}", ans);
        }
    }
}
