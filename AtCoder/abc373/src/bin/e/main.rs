use ac_library::{segtree::Segtree, Monoid};
use proconio::input;

struct Sum;
impl Monoid for Sum {
    type S = (u64, usize);

    fn identity() -> Self::S {
        (0, 0)
    }

    fn binary_operation(&(a, n): &Self::S, &(b, m): &Self::S) -> Self::S {
        (a + b, n + m)
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: u64,
        a: [u64; n],
    };

    let total = a.iter().sum::<u64>();
    let mut ai = Vec::new();
    for i in 0..n {
        ai.push((a[i], i));
    }
    ai.sort();
    let b = ai.iter().map(|&(a, _)| a).collect::<Vec<_>>();
    let mut seg = Segtree::<Sum>::new(n);
    for i in 0..n {
        seg.set(i, (b[i], 1));
    }

    let mut win = |a: u64, i: usize, x: u64| -> bool {
        assert!(x <= k - total);
        let p = b.partition_point(|&y| y <= (a + x));

        seg.set(i, (a + x + 1, 1));
        // 得票数が a + x を超える人の範囲 [j, p)
        // (p - j) * (a + x + 1) - sum[j, p) <= (k - total) - x
        let j = seg.min_left(p, |&(s, len)| {
            ((len as u64) * (a + x + 1)).saturating_sub(s) <= (k - total) - x
        });
        seg.set(i, (a, 1));

        // 自分より多く得票した人数 < m
        let mut g = n - j;
        if j <= i {
            // 自身を数えてるので引く
            g -= 1;
        }
        g < m
    };

    let mut ans = vec![String::new(); n];
    for (i, &(a, ans_i)) in ai.iter().enumerate() {
        ans[ans_i] = if win(a, i, 0) {
            "0".to_string()
        } else if !win(a, i, k - total) {
            "-1".to_string()
        } else {
            let mut ng = 0;
            let mut ok = k - total;
            while ok - ng > 1 {
                let mid = (ok + ng) / 2;
                if win(a, i, mid) {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ok.to_string()
        };
    }

    println!("{}", ans.join(" "));
}
