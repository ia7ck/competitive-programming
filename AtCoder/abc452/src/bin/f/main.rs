use ac_library::{Additive, Segtree};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [Usize1; n],
    };

    let geq = solve(n, k, &p);
    let gt = solve(n, k + 1, &p);
    let ans = geq - gt;

    println!("{}", ans);
}

// inv >= k
fn solve(n: usize, k: usize, p: &Vec<usize>) -> usize {
    let mut res = 0;
    let mut seg = Segtree::<Additive<usize>>::new(n);
    let mut inv = 0;
    let mut l = 0;
    for r in 0..n {
        inv += seg.prod((p[r] + 1)..);
        seg.set(p[r], seg.get(p[r]) + 1);
        while l <= r && inv >= k {
            inv -= seg.prod(..p[l]);
            seg.set(p[l], seg.get(p[l]) - 1);
            l += 1;
        }
        res += l;
    }
    res
}
