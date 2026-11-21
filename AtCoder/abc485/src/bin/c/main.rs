use ac_library::{Max, Min, Segtree};
use proconio::input;

fn main() {
    input! {
        n: usize,
        c: [usize; n],
    };

    let mut left = vec![INF; n];
    let mut seg = Segtree::<Max<isize>>::new(n + 1);
    for i in 0..n {
        seg.set(c[i], -INF);

        let d = i as isize - seg.all_prod();
        left[i] = left[i].min(d);

        seg.set(c[i], i as isize);
    }

    let mut right = vec![-INF; n];
    let mut seg = Segtree::<Min<isize>>::new(n + 1);
    for i in (0..n).rev() {
        seg.set(c[i], INF);

        let d = seg.all_prod() - i as isize;
        right[i] = right[i].max(d);

        seg.set(c[i], i as isize);
    }

    let ans = (0..n).map(|i| left[i].min(right[i])).collect::<Vec<_>>();
    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

const INF: isize = isize::MAX / 3;
