use ac_library::{Max, Min, Segtree};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        xy: [(i64, i64); n],
    };

    // s := x + y
    // t := x - y
    let mut seg_s_min = Segtree::<Min<i64>>::new(n);
    let mut seg_s_max = Segtree::<Max<i64>>::new(n);
    let mut seg_t_min = Segtree::<Min<i64>>::new(n);
    let mut seg_t_max = Segtree::<Max<i64>>::new(n);
    for (i, &(x, y)) in xy.iter().enumerate() {
        let s = x + y;
        let t = x - y;
        seg_s_min.set(i, s);
        seg_s_max.set(i, s);
        seg_t_min.set(i, t);
        seg_t_max.set(i, t);
    }
    for _ in 0..q {
        input! {
            op: u8,
        };

        if op == 1 {
            input! {
                i: Usize1,
                x: i64,
                y: i64,
            };
            let s = x + y;
            let t = x - y;
            seg_s_min.set(i, s);
            seg_s_max.set(i, s);
            seg_t_min.set(i, t);
            seg_t_max.set(i, t);
        } else {
            input! {
                l: Usize1,
                r: Usize1,
                x: i64,
                y: i64,
            };
            let s = x + y;
            let t = x - y;
            let s_min = seg_s_min.prod(l..(r + 1));
            let s_max = seg_s_max.prod(l..(r + 1));
            let t_min = seg_t_min.prod(l..(r + 1));
            let t_max = seg_t_max.prod(l..(r + 1));
            let ans = s
                .abs_diff(s_min)
                .max(s.abs_diff(s_max))
                .max(t.abs_diff(t_min))
                .max(t.abs_diff(t_max));
            println!("{}", ans);
        }
    }
}
