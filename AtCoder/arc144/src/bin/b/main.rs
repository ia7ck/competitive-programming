use proconio::input;

fn main() {
    input! {
        n: usize,
        a: u64,
        b: u64,
        c: [u64; n],
    };

    let mut lb = 0;
    let mut ub = 1_000_000_000 + 1;
    // lb <= ans < ub
    while ub - lb > 1 {
        let mid = (ub + lb) / 2;
        let mut s = 0;
        let mut t = 0;
        for &x in &c {
            if x < mid {
                s += (mid - x + (a - 1)) / a;
            } else {
                t += (x - mid) / b;
            }
        }
        if s <= t {
            lb = mid;
        } else {
            ub = mid;
        }
    }
    println!("{}", lb);
}
