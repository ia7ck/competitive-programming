use std::collections::HashMap;

use proconio::{input, marker::Bytes};

const N: usize = 50;

const PRIMES: [u64; N] = [
    999998957, 999998959, 999998971, 999998981, 999999001, 999999017, 999999029, 999999043,
    999999059, 999999067, 999999103, 999999107, 999999113, 999999131, 999999137, 999999151,
    999999163, 999999181, 999999191, 999999193, 999999197, 999999223, 999999229, 999999323,
    999999337, 999999353, 999999391, 999999433, 999999487, 999999491, 999999503, 999999527,
    999999541, 999999587, 999999599, 999999607, 999999613, 999999667, 999999677, 999999733,
    999999739, 999999751, 999999757, 999999761, 999999797, 999999883, 999999893, 999999929,
    999999937, 1000000007,
];

fn f(s: &[u8]) -> [u64; N] {
    let mut result = [0; N];
    for i in 0..N {
        for d in s {
            result[i] = (result[i] * 10 + u64::from(d - b'0')) % PRIMES[i];
        }
    }
    result
}

fn main() {
    input! {
        n: usize,
        a: [Bytes; n],
    };

    let mut xs = Vec::new();
    let mut count = HashMap::new();
    for a in &a {
        let x = f(a);
        xs.push(x.clone());
        *count.entry(x).or_insert(0) += 1;
    }

    let mut ans = 0_usize;
    for x in &xs {
        for y in &xs {
            let mut z = [0; N];
            for i in 0..N {
                z[i] = (x[i] * y[i]) % PRIMES[i];
            }
            ans += count.get(&z).copied().unwrap_or(0);
        }
    }
    println!("{}", ans);
}
