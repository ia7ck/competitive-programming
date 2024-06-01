use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    const M: usize = 1_000_000;
    let mut c = vec![0; M + 1];
    for x in a {
        c[x] += 1;
    }
    let mut cum_sum = vec![0; M + 1];
    for i in 1..=M {
        cum_sum[i] = cum_sum[i - 1] + c[i];
    }

    let mut ans = 0;
    for x in 1..=M {
        if c[x] == 0 {
            continue;
        }
        for d in 1..=(M / x) {
            // d = y / x
            // d * x ≦ y < (d + 1) * x
            let r = ((d + 1) * x - 1).min(M);
            ans += c[x] * d * (cum_sum[r] - cum_sum[d * x - 1]);
        }
        ans -= c[x] * (c[x] + 1) / 2;
    }
    
    println!("{}", ans);
}
