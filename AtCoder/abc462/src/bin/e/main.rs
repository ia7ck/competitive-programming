use proconio::input;

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            a: i64,
            b: i64,
            x: i64,
            y: i64,
        };

        solve(a, b, x, y);
    }
}

fn solve(a: i64, b: i64, x: i64, y: i64) {
    let x = x.abs();
    let y = y.abs();

    let mut ans = 0;
    let d = x.min(y);

    // (0, 0) -> (d, d)
    ans += a.min(b) * d * 2;

    if x > d {
        // (d, d) -> (x, d)
        let e = x - d;
        let f = (e + 1) / 2;
        let mut cost = a * f + b * (e - f);
        if e % 2 == 0 {
            cost = cost.min(a.min(b) * e * 2);
        } else {
            cost = cost.min(a * (e * 2 - 1));
            cost = cost.min(b * (e * 2 + 1));
        }
        ans += cost;
    } else if y > d {
        // (d, d) -> (d, y)
        let e = y - d;
        let f = (e + 1) / 2;
        let mut cost = a * (e - f) + b * f;
        if e % 2 == 0 {
            cost = cost.min(a.min(b) * e * 2);
        } else {
            cost = cost.min(a * (e * 2 + 1));
            cost = cost.min(b * (e * 2 - 1));
        }

        ans += cost;
    }

    println!("{}", ans);
}
