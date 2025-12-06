use ac_library::{Max, Segtree};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
    };

    let mut q = vec![0; n];
    for i in 0..n {
        q[p[i]] = i;
    }

    let mut seg = Segtree::<Max<usize>>::new(n);
    for i in 0..n {
        seg.set(i, p[i]);
    }
    let ans = f(0, n, &p, &q, &seg).unwrap();
    println!("{}", ans.d_sum);
}

struct F {
    p_max: usize,
    d_sum: usize,
}

fn f(l: usize, r: usize, p: &Vec<usize>, q: &Vec<usize>, seg: &Segtree<Max<usize>>) -> Option<F> {
    assert!(l <= r);
    if l == r {
        return None;
    }
    if l + 1 == r {
        return Some(F {
            p_max: p[l],
            d_sum: 0,
        });
    }

    // let p_max = p[l..r].iter().max().copied().unwrap();
    let p_max = seg.prod(l..r);
    let i = q[p_max];

    let mut d_sum = 0;
    if let Some(left) = f(l, i, p, q, seg) {
        let j = q[left.p_max];
        d_sum = d_sum.max(left.d_sum + i.abs_diff(j))
    }
    if let Some(right) = f(i + 1, r, p, q, seg) {
        let j = q[right.p_max];
        d_sum = d_sum.max(right.d_sum + i.abs_diff(j))
    }

    Some(F { p_max, d_sum })
}
