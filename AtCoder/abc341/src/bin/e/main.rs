use proconio::{
    input,
    marker::{Bytes, Usize1},
};
use segment_tree::SegmentTree;

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Bytes,
    };

    let mut seg = SegmentTree::new(n - 1, 0_usize, |x, y| x + y);
    for i in 0..(n - 1) {
        seg.update(i, usize::from(s[i] == s[i + 1]));
    }

    for _ in 0..q {
        input! {
            op: u8,
            l: Usize1,
            r: Usize1,
        };
        if op == 1 {
            if l >= 1 {
                seg.update(l - 1, 1 - seg.get(l - 1));
            }
            if r < n - 1 {
                seg.update(r, 1 - seg.get(r));
            }
        } else {
            let sum = seg.fold(l..r);
            if sum == 0 {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
