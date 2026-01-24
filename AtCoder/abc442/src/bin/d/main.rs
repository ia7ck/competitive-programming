use ac_library::{Additive, Segtree};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u32; n],
    };

    let mut seg = Segtree::<Additive<u32>>::new(n);
    for i in 0..n {
        seg.set(i, a[i]);
    }
    for _ in 0..q {
        input! {
            op: u8,
        };

        if op == 1 {
            input! {
                x: Usize1,
            };
            let a1 = seg.get(x);
            let a2 = seg.get(x + 1);
            seg.set(x, a2);
            seg.set(x + 1, a1);
        } else {
            input! {
                l: Usize1,
                r: Usize1,
            };
            let ans = seg.prod(l..=r);
            println!("{}", ans);
        }
    }
}
