use proconio::{input, marker::Bytes};

use run_length::RunLength;

const M: usize = 998244353;

macro_rules! add {
    ($a: expr, $b: expr) => {
        $a = ($a + $b % M) % M;
    };
}

fn main() {
    input! {
        n: usize,
        s: Bytes,
    };

    for i in 1..n {
        if s[i - 1] != b'1' && s[i] != b'1' {
            println!("-1");
            return;
        }
    }

    let rl = RunLength::new(s.iter()).collect::<Vec<_>>();
    let mut ans = 0;
    for i in (0..rl.len()).rev() {
        let (x, xl) = rl[i];
        if x == &b'1' {
            if i + 1 == rl.len() {
                add!(ans, xl);
            } else {
                let (y, _) = rl[i + 1];
                assert_ne!(y, &b'1');
                add!(ans, xl + (y - b'0' - 1) as usize * ans);
            }
        } else {
            assert_eq!(xl, 1);
            add!(ans, 1);
        }
    }
    println!("{}", (ans + M - 1) % M);
}
