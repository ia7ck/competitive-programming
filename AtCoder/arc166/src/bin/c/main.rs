use proconio::input;

const M: u64 = 998244353;

fn pow(a: u64, x: u64) -> u64 {
    if x == 0 {
        1
    } else if x % 2 == 0 {
        pow(a * a % M, x / 2)
    } else {
        a * pow(a, x - 1) % M
    }
}

fn main() {
    input! {
        t: usize,
    };

    // h=4, w=7
    // +-+-+-+-+-+-+-+
    // | | | | | | | |
    // +-+-+-+-+-+-+-+
    // | | | | | | | |
    // +-+-+-+-+-+-+-+
    // | | | | | | | |
    // +-+-+-+-+-+-+-+
    // | | | | | | | |
    // +-+-+-+-+-+-+-+

    // 2, 4, 6, 8, 9, 9, 9, 8, 6, 4, 2

    const N: usize = 2_000_000 + 2;
    let mut f = vec![0; N];
    f[2] = 2;
    f[3] = 3;
    for i in 4..N {
        f[i] = (f[i - 2] + f[i - 1]) % M;
    }
    let mut cum_mul = vec![0; N];
    // cul_mul[i * 2] := f[2] * f[4] * ... * f[i * 2]
    cum_mul[2] = f[2];
    for i in (4..N).step_by(2) {
        cum_mul[i] = f[i] * cum_mul[i - 2] % M;
    }

    for _ in 0..t {
        input! {
            h: usize,
            w: usize,
        };

        let l = h.min(w);

        // let mut ans = 1;
        // for i in 1..=l {
        //     ans *= f[i * 2].pow(2) % M;
        //     ans %= M;
        // }
        // ans *= pow(f[l * 2 + 1], h.abs_diff(w) as u64) % M;
        // ans %= M;

        let ans = cum_mul[l * 2].pow(2) % M * pow(f[l * 2 + 1], h.abs_diff(w) as u64) % M;
        println!("{}", ans);
    }
}
