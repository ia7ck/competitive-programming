use proconio::{input, marker::Bytes};

fn main() {
    input! {
        s: Bytes,
    };

    let mut memo = [0_u64; 1 << 10];
    memo[0] = 1;
    let mut f = [0; 10];
    let mut ans = 0;
    for b in s {
        f[(b - b'0') as usize] += 1;
        let mut k = 0;
        for i in 0..10 {
            if f[i] % 2 == 1 {
                k ^= 1 << i;
            }
        }
        ans += memo[k];
        memo[k] += 1;
    }

    println!("{}", ans);
}
