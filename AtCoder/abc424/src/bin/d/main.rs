use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            h: usize,
            w: usize,
            s: [Chars; h],
        };

        solve(h, w, s);
    }
}

macro_rules! chmin {
    ($a: expr, $b: expr) => {
        $a = $a.min($b);
    };
}

fn solve(h: usize, w: usize, s: Vec<Vec<char>>) {
    // white: 0
    // black: 1

    const INF: u32 = u32::MAX / 2;
    let mut dp = vec![INF; 1 << w];
    dp[bin(&s[0])] = 0;
    for i in 1..h {
        let row = bin(&s[i]);
        let mut new_dp = vec![INF; 1 << w];
        for state in 0..(1 << w) {
            if dp[state] == INF {
                continue;
            }
            for bit in 0..(1 << w) {
                // subset
                if (row & bit) == bit {
                    let mut ok = true;
                    for j in 0..(w - 1) {
                        // 2x2 blacks
                        if state >> j & 1 == 1
                            && state >> (j + 1) & 1 == 1
                            && bit >> j & 1 == 1
                            && bit >> (j + 1) & 1 == 1
                        {
                            ok = false;
                        }
                    }
                    if ok {
                        let paint = (row ^ bit).count_ones();
                        chmin!(new_dp[bit], dp[state] + paint);
                    }
                }
            }
        }
        dp = new_dp;
    }

    let ans = dp.iter().min().copied().unwrap();
    assert_ne!(ans, INF);
    println!("{}", ans);
}

fn bin(s: &Vec<char>) -> usize {
    let mut b = 0;
    for &c in s {
        b = b * 2;
        if c == '#' {
            b += 1;
        }
    }
    b
}
