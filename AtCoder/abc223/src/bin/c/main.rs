use input_i_scanner::{scan_with, InputIScanner};

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let n = scan_with!(_i_i, usize);
    let ab = scan_with!(_i_i, (f64, f64); n);

    let f = |mut t: f64| -> f64 {
        let mut res = 0.0;
        for &(a, b) in &ab {
            if t >= a / b {
                t -= a / b;
                res += a;
            } else {
                res += b * t; // b * t < a
                break;
            }
        }
        res
    };

    let g = |mut t: f64| -> f64 {
        let mut res = 0.0;
        for &(a, b) in ab.iter().rev() {
            if t >= a / b {
                t -= a / b;
                res += a;
            } else {
                res += b * t; // b * t < a
                break;
            }
        }
        res
    };

    let mut total = 0.0;
    for &(a, _) in &ab {
        total += a;
    }
    let mut ng = 0.0;
    let mut ok = 1e10;
    for _ in 0..100 {
        let mid = (ng + ok) / 2.0;
        if f(mid) + g(mid) >= total {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", f(ok));
}
