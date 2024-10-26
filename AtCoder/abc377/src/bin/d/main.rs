use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        lr: [(usize, usize); n],
    };

    let mut seg_by_r = vec![vec![]; m + 1];
    for (l, r) in lr {
        seg_by_r[r].push(l);
    }

    let mut l_max = 0;
    let mut ans = 0;
    for r in 1..=m {
        for &l in &seg_by_r[r] {
            l_max = l_max.max(l);
        }
        ans += r - l_max;
    }

    println!("{}", ans);
}
