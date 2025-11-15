use ac_library::{Additive, Segtree};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    };

    const M: usize = 500_000 + 1;
    let mut value = Segtree::<Additive<usize>>::new(M);
    let mut count = Segtree::<Additive<usize>>::new(M);
    for &a in &a {
        let x = value.get(a);
        value.set(a, x + a);
        let x = count.get(a);
        count.set(a, x + 1);
    }

    let mut a = a;
    for _ in 0..q {
        input! {
            op: u8,
        };

        if op == 1 {
            input! {
                x: Usize1,
                y: usize,
            };

            value.set(a[x], value.get(a[x]) - a[x]);
            count.set(a[x], count.get(a[x]) - 1);
            a[x] = y;
            value.set(a[x], value.get(a[x]) + a[x]);
            count.set(a[x], count.get(a[x]) + 1);
        } else {
            input! {
                l: usize,
                r: usize,
            };

            if l >= r {
                println!("{}", l * n);
            } else {
                let less_count = count.prod(0..l);
                let sum = value.prod(l..=r);
                let greater_count = count.prod((r + 1)..);
                let ans = less_count * l + sum + greater_count * r;
                println!("{}", ans);
            }
        }
    }
}
