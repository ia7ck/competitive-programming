use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        _m: usize,
    };

    let mut p = vec![0; n];
    let mut f = vec![HashSet::new(); n];
    for i in 0..n {
        input! {
            p_: u32,
            c: usize,
            f_: [u32; c],
        };
        p[i] = p_;
        f[i] = f_.into_iter().collect();
    }

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            if p[i] >= p[j] && f[j].is_superset(&f[i]) {
                if p[i] > p[j] || f[j].len() > f[i].len() {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
