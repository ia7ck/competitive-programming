use ac_library::{Additive, Segtree};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    };

    let mut a = vec![0; n];
    let mut seg = Segtree::<Additive<usize>>::new(300_000 + 1);
    seg.set(0, n);
    let mut del = 0;
    for _ in 0..q {
        input! {
            op: u8,
        };

        if op == 1 {
            input! {
                x: Usize1,
            };

            let y = seg.get(a[x]);
            seg.set(a[x], y - 1);
            a[x] += 1;
            let y = seg.get(a[x]);
            seg.set(a[x], y + 1);

            if seg.prod((del + 1)..) == n {
                del += 1;
            }
        } else {
            input! {
                y: usize,
            };

            let ans = seg.prod((y + del).min(300_000)..);
            println!("{}", ans);
        }
    }
}
