use ac_library::{Additive, Segtree};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u64; n],
    };

    let mut seg = Segtree::<Additive<u64>>::new(n);
    for i in 0..n {
        seg.set(i, a[i]);
    }

    for _ in 0..q {
        input! {
            op: u8,
        };

        if op == 1 {
            input! {
                i: Usize1,
                x: u64,
            };

            seg.set(i, x);
        } else if op == 2 {
            input! {
                l: Usize1,
                t: u64,
            };

            let r = seg.max_right(l, |&s| s < t);
            debug_assert!(seg.prod(l..r) < t);
            if r < n {
                debug_assert!(seg.prod(l..=r) >= t);
            }

            println!("{}", r + 1);
        } else {
            input! {
                r: Usize1,
                t: u64,
            };

            let l = seg.min_left(r + 1, |&s| s < t);
            debug_assert!(seg.prod(l..=r) < t);
            if l > 0 {
                debug_assert!(seg.prod((l - 1)..=r) >= t);
            }

            println!("{}", l);
        }
    }
}
