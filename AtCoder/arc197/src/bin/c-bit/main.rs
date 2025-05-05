use fenwick_tree::FenwickTree;
use proconio::input;

fn main() {
    input! {
        q: usize,
        ab: [(usize, i64); q],
    };

    const M: usize = 3_000_000;
    let mut f = FenwickTree::new(M, 0_i64);
    for i in 0..M {
        f.add(i, 1);
    }
    for (a, b) in ab {
        if a < M && f.sum(a..=a) == 1 {
            for x in (a..M).step_by(a) {
                if f.sum(x..=x) == 1 {
                    f.add(x, -1);
                }
            }
        }
        let mut ok = 0;
        let mut ng = M + 1;
        while ng - ok > 1 {
            let mid = (ng + ok) / 2;
            if f.sum(0..mid) <= b {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        println!("{}", ok);
    }
}
