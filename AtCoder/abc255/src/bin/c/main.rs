use proconio::input;

fn main() {
    input! {
        x: i64,
        a: i64,
        d: i64,
        n: i64,
    };

    let kth = |k| {
        assert!(k >= 1);
        assert!(k <= n);
        a + d * (k - 1)
    };

    if d >= 0 {
        if x <= kth(1) {
            println!("{}", kth(1) - x);
            return;
        }

        if kth(n) < x {
            println!("{}", x - kth(n));
            return;
        }

        let mut ng = 1;
        let mut ok = n;
        // kth(ng) < x <= kth(ok)
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if kth(mid) < x {
                ng = mid;
            } else {
                ok = mid;
            }
        }
        let mut ans = kth(ok) - x;
        if kth(ng) < x {
            ans = ans.min(x - kth(ng));
        }
        println!("{}", ans);
    } else {
        if x >= kth(1) {
            println!("{}", x - kth(1));
            return;
        }

        if kth(n) > x {
            println!("{}", kth(n) - x);
            return;
        }

        let mut ng = 1;
        let mut ok = n;
        // kth(ng) > x >= kth(ok)
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if kth(mid) > x {
                ng = mid;
            } else {
                ok = mid;
            }
        }
        let mut ans = x - kth(ok);
        if kth(ng) > x {
            ans = ans.min(kth(ng) - x);
        }
        println!("{}", ans);
    }
}
