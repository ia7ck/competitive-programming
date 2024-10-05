use proconio::input;

fn main() {
    input! {
        n: usize,
        k: [u64; n],
    };

    let mut ans = u64::MAX;
    for bits in 0..(1 << n) {
        let mut a = 0;
        let mut b = 0;
        for i in 0..n {
            if bits >> i & 1 == 1 {
                a += k[i];
            } else {
                b += k[i];
            }
        }
        ans = ans.min(a.max(b));
    }

    println!("{}", ans);
}
