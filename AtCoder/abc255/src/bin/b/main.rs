use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; k],
        xy: [(f64, f64); n],
    };

    let a: Vec<usize> = a.into_iter().map(|x| x - 1).collect();
    let a_set: HashSet<usize> = a.iter().copied().collect();

    let mut lb = 0.0;
    let mut ub = 1e6;
    for _ in 0..1000 {
        let d = (lb + ub) / 2.0;
        let mut ok = true;
        for i in 0..n {
            if a_set.contains(&i) {
                continue;
            }
            let (x, y) = xy[i];
            let mut found = false;
            for &j in &a {
                let (xx, yy) = xy[j];
                if (x - xx).hypot(y - yy) <= d {
                    found = true;
                    break;
                }
            }
            if !found {
                ok = false;
            }
        }
        if ok {
            ub = d;
        } else {
            lb = d;
        }
    }

    println!("{}", lb);
}
