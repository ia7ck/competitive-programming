use proconio::input;
use segment_tree::SegmentTree;

fn main() {
    input! {
        n: usize,
        l: usize,
        d: usize,
    };

    let mut deal = vec![0.0; l + d + 1];
    deal[1] += 1.0 / d as f64;
    deal[d + 1] -= 1.0 / d as f64;
    for i in 1..l {
        deal[i] += deal[i - 1];
        let p = deal[i] / d as f64;
        deal[i + 1] += p;
        deal[i + d + 1] -= p;
    }
    for i in l..=(l + d) {
        deal[i] += deal[i - 1];
    }
    // p_deal[..l] values are not matter?

    let deal = {
        let mut seg = SegmentTree::new(l + d + 1, 0.0, |&x, &y| x + y);
        for i in l..=(l + d) {
            seg.update(i, deal[i]);
        }
        seg
    };

    let mut win = SegmentTree::new(n + 1, 0.0, |&x, &y| x + y);
    for x in (0..=n).rev() {
        // roll dice
        let p = win.fold((x + 1)..((x + d).min(n) + 1)) / d as f64;

        // not roll dice
        // y > n
        let q = if l + d <= n {
            0.0
        } else {
            // l <= n < l + d
            deal.fold((n + 1)..(l + d))
        };
        // x > y
        let r = if x <= l {
            0.0
        } else {
            // l < x
            deal.fold(l..x.min(l + d))
        };

        win.update(x, p.max(q + r));
    }

    println!("{}", win.get(0));
}
