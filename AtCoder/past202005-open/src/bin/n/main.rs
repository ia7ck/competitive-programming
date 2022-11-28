use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        txy: [(u8, usize, usize); q],
    };

    let mut a = vec![0; n];
    for i in 0..n {
        a[i] = i;
    }
    // a[i] > a[i + 1] となる i
    let mut pos = BTreeSet::new();
    for (t, x, y) in txy {
        if t == 1 {
            let x = x - 1;
            pos.remove(&(x + 1));
            a.swap(x, x + 1);
            if x >= 1 {
                if a[x - 1] > a[x] {
                    pos.insert(x - 1);
                } else {
                    pos.remove(&(x - 1));
                }
            }
            if a[x] > a[x + 1] {
                pos.insert(x);
            } else {
                pos.remove(&x);
            }
            if x + 2 < n {
                if a[x + 1] > a[x + 2] {
                    pos.insert(x + 1);
                } else {
                    pos.remove(&(x + 1));
                }
            }
        } else {
            let (x, y) = (x - 1, y - 1);
            while let Some(&p) = pos.range(x..y).next() {
                assert!(a[p] > a[p + 1]);
                a.swap(p, p + 1);
                if p >= 1 {
                    if a[p - 1] > a[p] {
                        pos.insert(p - 1);
                    } else {
                        pos.remove(&(p - 1));
                    }
                }
                pos.remove(&p); // a[p] < a[p + 1]
                if p + 2 < n {
                    if a[p + 1] > a[p + 2] {
                        pos.insert(p + 1);
                    } else {
                        pos.remove(&(p + 1));
                    }
                }
            }
        }
    }

    for i in 0..n {
        print!("{}", a[i] + 1);
        if i + 1 < n {
            print!(" ");
        }
    }
    println!();
}
