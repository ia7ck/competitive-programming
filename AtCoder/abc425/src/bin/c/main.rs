use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u64; n],
    };

    let mut cum_sum = vec![0; n];
    for i in 0..n {
        if i >= 1 {
            cum_sum[i] = cum_sum[i - 1];
        }
        cum_sum[i] += a[i];
    }

    let mut offset = 0;
    for _ in 0..q {
        input! {
            op: u8,
        };

        if op == 1 {
            input! {
                c: usize,
            };
            offset += c;
        } else {
            input! {
                l: Usize1,
                r: Usize1,
            };

            let l = (l + offset) % n;
            let r = (r + offset) % n;
            if l <= r {
                let mut ans = cum_sum[r];
                if l >= 1 {
                    ans -= cum_sum[l - 1];
                }
                println!("{}", ans);
            } else {
                let mut ans = 0;
                ans += cum_sum[n - 1] - cum_sum[l - 1];
                ans += cum_sum[r];
                println!("{}", ans);
            }
        }
    }
}
