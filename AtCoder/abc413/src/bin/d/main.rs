use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            a: [i64; n],
        };

        solve(n, a);
    }
}

fn solve(n: usize, a: Vec<i64>) {
    if a.iter().all(|&x| x > 0) {
        let mut b = a.clone();
        b.sort_unstable();

        let mut ans = true;
        for w in b.windows(3) {
            // w[1] / w[0] = w[2] / w[1]
            if w[1] * w[1] == w[0] * w[2] {
                // ok
            } else {
                ans = false;
            }
        }

        if ans {
            println!("Yes");
        } else {
            println!("No");
        }
    } else if a.iter().all(|&x| x < 0) {
        solve(n, a.into_iter().map(|x| -x).collect())
    } else {
        // r < 0
        // r <= -1 ??

        let mut p = Vec::new();
        let mut q = Vec::new();
        for x in a {
            if x > 0 {
                p.push(x);
            } else {
                q.push(x);
            }
        }
        p.sort_unstable_by_key(|&x| x.abs());
        q.sort_unstable_by_key(|&x| x.abs());

        for (p, q) in [(&p, &q), (&q, &p)] {
            // p[0], q[0], p[1], q[1], ...
            let mut p = p.into_iter().collect::<VecDeque<_>>();
            let mut q = q.into_iter().collect::<VecDeque<_>>();
            let mut b = Vec::new();
            loop {
                match (p.pop_front(), q.pop_front()) {
                    (None, None) => break,
                    (None, Some(_)) => break,
                    (Some(x), None) => {
                        b.push(x);
                    }
                    (Some(x), Some(y)) => {
                        b.push(x);
                        b.push(y);
                    }
                }
            }
            if b.len() == n {
                let mut ans = true;
                for w in b.windows(3) {
                    // w[1] / w[0] = w[2] / w[1]
                    if w[1] * w[1] == w[0] * w[2] {
                        // ok
                    } else {
                        ans = false;
                    }
                }

                if ans {
                    println!("Yes");
                    return;
                }
            }
        }

        println!("No");
    }
}
