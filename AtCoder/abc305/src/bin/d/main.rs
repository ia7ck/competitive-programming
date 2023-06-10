use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        q: usize,
        lr: [(u64, u64); q],
    };

    let mut sl = vec![0; n];
    for i in 0..n {
        if i % 2 == 0 {
            if i >= 2 {
                sl[i] = sl[i - 1] + (a[i] - a[i - 1]);
            }
        } else {
            sl[i] = sl[i - 1];
        }
    }

    let solve = |x: u64| -> u64 {
        match a.binary_search(&x) {
            Ok(i) => {sl[i]},
            Err(i) => {
                assert!(i >= 1);
                if (i - 1) % 2 == 0 {
                    sl[i - 1]
                } else {
                    sl[i - 1] + (x - a[i - 1])
                }
            },
        }
    };

    for (l, r) in lr {
        let ans = solve(r) - solve(l);
        println!("{}", ans);
    }

}
