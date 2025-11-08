use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        h: [u32; n],
        b: [u32; m],
    };

    let mut h = h;
    h.sort_unstable();

    let mut b = b
        .into_iter()
        .enumerate()
        .map(|(i, b)| (b, i))
        .collect::<BTreeSet<_>>();

    let mut ans = 0;
    for h in h {
        if let Some(e) = b.range((h, 0)..).next().copied() {
            ans += 1;
            b.remove(&e);
        }
    }

    if ans >= k {
        println!("Yes");
    } else {
        println!("No");
    }
}
