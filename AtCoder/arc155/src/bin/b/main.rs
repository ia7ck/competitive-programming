use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        q: usize,
        a: i64,
        b: i64,
    };
    // eprintln!("a = {}, b = {}, a + b = {}", a, b, a + b);

    const INF: i64 = std::i64::MAX / 2;

    let mut set = BTreeSet::new();
    set.insert((a + b, a, b));
    set.insert((a - b, a, b));
    for _ in 0..q {
        input! {
            t: u8,
            a: i64,
            b: i64,
        };

        // eprintln!("{}, a = {}, b = {}, a + b = {}", t, a, b, a + b);

        if t == 1 {
            set.insert((a + b, a, b));
            set.insert((a - b, a, b));
        } else {
            let mut ans = INF;
            if set.range((a, 0, 0)..=(b, INF, INF)).next().is_some() {
                ans = ans.min(0);
            }
            if let Some(&(_, aa, bb)) = set.range(..(a, INF, INF)).last() {
                ans = ans.min(((a - aa).abs() - bb).abs());
            }
            if let Some(&(_, aa, bb)) = set.range((b, 0, 0)..).next() {
                ans = ans.min(((b - aa).abs() - bb).abs());
            }
            assert_ne!(ans, INF);
            println!("{}", ans);
        }
    }
}
