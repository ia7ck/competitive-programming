use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        points: [(i64, i64); n],
    };

    if k == 1 {
        println!("Infinity");
        return;
    }

    // (y2 - y1) * x - (x2 - x1) * y + (x2 - x1) * y1 - (y2 - y1) * x1 = 0
    let mut x_axis = Vec::new();
    let mut y_axis = Vec::new();
    let mut abc = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];
            if x1 == x2 {
                x_axis.push(x1);
            } else if y1 == y2 {
                y_axis.push(y1);
            } else {
                let a = y2 - y1;
                let b = -(x2 - x1);
                let c = (x2 - x1) * y1 - (y2 - y1) * x1;
                let g = gcd(a.abs() as u64, b.abs() as u64);
                let g = gcd(g, c.abs() as u64);
                let a = a / g as i64;
                let b = b / g as i64;
                let c = c / g as i64;
                if a > 0 {
                    abc.push((a, b, c));
                } else if a < 0 {
                    abc.push((-a, -b, -c));
                } else {
                    unreachable!();
                }
            }
        }
    }

    x_axis.sort();
    x_axis.dedup();
    y_axis.sort();
    y_axis.dedup();
    abc.sort();
    abc.dedup();

    let mut ans = 0;
    for x1 in x_axis {
        let mut m = 0;
        for &(x, _) in &points {
            if x == x1 {
                m += 1;
            }
        }
        if m >= k {
            ans += 1;
        }
    }
    for y1 in y_axis {
        let mut m = 0;
        for &(_, y) in &points {
            if y == y1 {
                m += 1;
            }
        }
        if m >= k {
            ans += 1;
        }
    }
    for (a, b, c) in abc {
        let mut m = 0;
        for &(x, y) in &points {
            if a * x + b * y + c == 0 {
                m += 1;
            }
        }
        if m >= k {
            ans += 1;
        }
    }

    println!("{}", ans);
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
