use std::collections::HashMap;

use proconio::{input, marker::Chars};

macro_rules! add {
    ($a: expr, $b: expr) => {
        $a = ($a + $b) % 998_244_353;
    };
}

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };

    let (h, w, s) = if w <= h {
        (h, w, s)
    } else {
        let mut t = vec![vec!['?'; h]; w];
        for i in 0..h {
            for j in 0..w {
                t[j][i] = s[i][j];
            }
        }
        (w, h, t)
    };
    assert!(w <= 15);

    let ten = 10_u64.pow((w - 1) as u32);
    let mut dp = HashMap::new();
    {
        // key = 444...4
        let key = (0..w).fold(0_u64, |acc, _| acc * 10 + 4);
        dp.insert(key, 1_u64);
    }
    for i in 0..h {
        for j in 0..w {
            let mut new_dp = HashMap::new();
            for (k, v) in dp {
                for d in 1..=3 {
                    let put = s[i][j] == '?' || s[i][j].to_digit(10) == Some(d);
                    let d = u64::from(d);
                    let up = k / ten != d;
                    let left = j == 0 || k % 10 != d;
                    if put && up && left {
                        new_dp
                            .entry((k * 10 + d) % (ten * 10))
                            .and_modify(|x| {
                                add!(*x, v);
                            })
                            .or_insert(v);
                    }
                }
            }
            dp = new_dp;
        }
    }
    
    let mut ans = 0;
    for v in dp.values() {
        add!(ans, v);
    }
    println!("{}", ans);
}
