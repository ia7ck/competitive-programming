use proconio::input;

fn main() {
    input! {
        l: i64,
        r: i64,
        d: i64,
        u: i64,
    };

    let mut v = 0;
    for x in -1_000_000..=1_000_000 {
        if x % 2 == 0 && l <= x && x <= r {
            let d = (-1 * x.abs()).max(d);
            let u = x.abs().min(u);
            if d <= u {
                v += u - d + 1;
            }
        }
    }

    let mut h = 0;
    for y in -1_000_000..=1_000_000 {
        if y % 2 == 0 && d <= y && y <= u {
            let l = (-1 * y.abs()).max(l);
            let r = y.abs().min(r);
            if l <= r {
                h += r - l + 1;
            }
        }
    }

    let mut c = 0;
    for x in -1_000_000_i64..=1_000_000 {
        let ys = if x == 0 {
            vec![0]
        } else {
            vec![-1 * x.abs(), x.abs()]
        };
        for y in ys {
            if x.abs().max(y.abs()) % 2 == 0 && l <= x && x <= r && d <= y && y <= u {
                c += 1;
            }
        }
    }

    let ans = v + h - c;
    println!("{}", ans);
}
