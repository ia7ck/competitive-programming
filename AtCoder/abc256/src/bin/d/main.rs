use proconio::input;

fn main() {
    input! {
        n: usize,
        lr: [(usize, usize); n],
    };

    const M: usize = 2_000_00;
    let mut cum_sum = vec![0_i64; M + 2];
    for (l, r) in lr {
        cum_sum[l] += 1;
        cum_sum[r] -= 1;
    }
    for i in 1..=(M + 1) {
        cum_sum[i] += cum_sum[i - 1];
    }
    assert_eq!(cum_sum[M + 1], 0);
    let mut ans = Vec::new();
    let mut ok = false;
    let mut left = 0;
    for i in 1..=(M + 1) {
        if cum_sum[i] == 0 {
            if ok {
                assert_ne!(left, 0);
                ans.push((left, i));
                ok = false;
                left = 0; // ?
            }
        } else {
            if !ok {
                ok = true;
                left = i;
            }
        }
    }
    for (l, r) in ans {
        println!("{} {}", l, r);
    }
}
