use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    };

    let cost = |x: u64| {
        let mut s = 0;
        let mut t = 0;
        for &a in &a {
            if a < x {
                s += x - a;
            }
            if x + 1 < a {
                t += a - (x + 1)
            }
        }
        s.max(t)
    };

    let mut lb = 0;
    let mut ub = 1_000_000_000 + 1;
    while ub - lb > 2 {
        let x1 = (lb * 2 + ub) / 3;
        let x2 = (lb + ub * 2) / 3;
        if cost(x1) < cost(x2) {
            ub = x2;
        } else {
            lb = x1;
        }
    }

    let ans = cost(lb).min(cost(lb + 1));
    println!("{}", ans);
}
