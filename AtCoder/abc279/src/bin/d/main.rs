use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
    };

    let f = |x: f64| -> f64 {
        a / (1.0 + x).sqrt() + b * x
    };

    let mut ub = 1_000_000_000_000_000_000_u64;
    let mut lb = 0;
    while ub - lb > 2 {
        let x1 = (lb * 2 + ub) / 3;
        let x2 = (lb + ub * 2) / 3;
        if f(x1 as f64) > f(x2 as  f64) {
            lb = x1;
        } else {
            ub = x2;
        }
    }

    let ans = f(lb as f64).min(f((lb + 1) as f64)).min(f(ub as f64));
    println!("{}", ans);
}
