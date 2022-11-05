use std::collections::HashMap;

use proconio::input;

fn f(mut x: u32) -> Option<(u32, u32)> {
    let (mut t2, mut t3) = (0, 0);
    while x % 2 == 0 || x % 3 == 0 {
        if x % 2 == 0 {
            x /= 2;
            t2 += 1;
            continue;
        }
        if x % 3 == 0 {
            x /= 3;
            t3 += 1;
            continue;
        }
    }
    if x == 1 {
        Some((t2, t3))
    } else {
        None
    }
}

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };

    let mut dp = HashMap::<u32, u32>::new();
    for i in 0..30 {
        for j in 0..19 {
            let p2 = 2_u32.pow(i);
            let p3 = 3_u32.pow(j);
            if a[0] % p2 == 0 && (a[0] / p2) % p3 == 0 {
                dp.insert(a[0] / p2 / p3, i + j);
            }
        }
    }

    for i in 1..n {
        let mut next = HashMap::<u32, u32>::new();
        for (v, min) in dp {
            if a[i] % v == 0 {
                if let Some((t2, t3)) = f(a[i] / v) {
                    let e = next.entry(v).or_insert(min + (t2 + t3));
                    *e = (*e).min(min + (t2 + t3));
                }
            }
        }
        dp = next;
    }

    if let Some(ans) = dp.values().min() {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
