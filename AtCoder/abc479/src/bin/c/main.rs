use proconio::input;

fn main() {
    input! {
        n: usize,
        w: u64,
        ab: [(u64, u64); n],
    };

    let mut ans = 0;
    for bits in 0..(1 << n) {
        let mut a_sum = 0;
        let mut b_sum = 0;
        for i in 0..n {
            if bits >> i & 1 == 1 {
                let (a, b) = ab[i];
                a_sum += a;
                b_sum += b;
            }
        }
        if b_sum <= w {
            ans = ans.max(a_sum);
        }
    }

    println!("{}", ans);
}
