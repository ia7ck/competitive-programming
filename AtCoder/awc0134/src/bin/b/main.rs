use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        m: usize,
        lr: [(usize, usize); n],
        se: [(usize, usize); m],
    };

    // [l, r], [1, e]
    let f = |l: usize, r: usize, e: usize| -> usize {
        assert!(1 <= l && l <= r && r <= 7);
        assert!(e <= 7 * w);

        if e == 0 {
            return 0;
        }

        let k = (e - 1) / 7 * 7;
        assert!(k < e && e <= k + 7);

        let repeat = (r - l + 1) * (k / 7);
        let rr = r.min(e - k);
        // rr - l + 1
        let tail = rr.checked_sub(l).map(|d| d + 1).unwrap_or(0);
        repeat + tail
    };

    let mut ans = HashMap::<(usize, usize), usize>::new();
    for l in 1..=7 {
        for r in l..=7 {
            let mut point = (r - l + 1) * w;
            for &(s, e) in &se {
                let special = f(l, r, e) - f(l, r, s - 1);
                point += special;
            }
            ans.insert((l, r), point);
        }
    }
    for l in 1..=7 {
        for r in 1..l {
            let whole = ans[&(1, 7)];
            let sub = ans.get(&(r + 1, l - 1)).copied().unwrap_or(0);
            ans.insert((l, r), whole - sub);
        }
    }

    for (l, r) in lr {
        let ans = ans[&(l, r)];
        println!("{}", ans);
    }
}
