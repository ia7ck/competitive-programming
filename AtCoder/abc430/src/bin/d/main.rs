use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [u64; n],
    };

    let mut ans = 0_u64;
    let mut d = vec![u64::MAX; n + 1];
    let mut set = BTreeSet::from([(0_u64, 0_usize)]);
    for (x, i) in x.into_iter().zip(1..=n) {
        if let Some(&(lx, li)) = set.range(..(x, usize::MAX)).last() {
            d[i] = d[i].min(x - lx);
            if i == 1 {
                assert_eq!(li, 0);
                assert_eq!(d[li], u64::MAX);
            } else {
                ans -= d[li];
            }
            d[li] = d[li].min(x - lx);
            ans += d[li];
        }
        if let Some(&(rx, ri)) = set.range((x, 0)..).next() {
            d[i] = d[i].min(rx - x);
            ans -= d[ri];
            d[ri] = d[ri].min(rx - x);
            ans += d[ri];
        }
        assert_ne!(d[i], u64::MAX);
        ans += d[i];
        set.insert((x, i));

        println!("{}", ans);
    }
}
