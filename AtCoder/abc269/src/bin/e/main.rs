use std::io;
use std::io::BufReader;

use proconio::input;
use proconio::source::line::LineSource;

fn main() {
    let mut stdin = LineSource::new(BufReader::new(io::stdin()));
    input! {
        from &mut stdin,
        n: usize,
    };

    let mut query = |a: usize, b: usize, c: usize, d: usize| -> usize {
        println!("? {} {} {} {}", a, b, c, d);
        input! {
            from &mut stdin,
            t: isize,
        };
        assert_ne!(t, -1);
        t as usize
    };

    // row
    let mut lb = 1;
    let mut ub = n;
    while lb != ub {
        let mid = (lb + ub) / 2;
        // [lb, mid], [mid + 1, ub]
        let t = query(lb, mid, 1, n);
        if t < mid - lb + 1 {
            ub = mid;
        } else {
            lb = mid + 1;
        }
    }

    let r = lb;

    // column
    let mut lb = 1;
    let mut ub = n;
    while lb != ub {
        let mid = (lb + ub) / 2;
        // [lb, mid], [mid + 1, ub]
        let t = query(1, n, lb, mid);
        if t < mid - lb + 1 {
            ub = mid;
        } else {
            lb = mid + 1;
        }
    }

    let c = lb;

    println!("! {} {}", r, c);
}
