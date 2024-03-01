use proconio::input;

const M: usize = 998244353;

macro_rules! add {
    ($a: expr, $b: expr) => {
        $a = ($a + $b) % M;
    };
}

macro_rules! sub {
    ($a: expr, $b: expr) => {
        $a = (M + $a - $b % M) % M;
    };
}

fn main() {
    input! {
        n: usize,
        a1: usize,
        a2: usize,
        a3: usize,
    };

    // dp[less1][less2][less3][mod a1][mod a2][mod a3]
    let mut dp = vec![vec![vec![vec![vec![vec![0; a3]; a2]; a1]; 2]; 2]; 2];
    dp[0][0][0][0][0][0] = 1_usize; // ???
    for i in (0..=(n.ilog2() as usize)).rev() {
        let mut new_dp = vec![vec![vec![vec![vec![vec![0; a3]; a2]; a1]; 2]; 2]; 2];
        for less1 in [false, true] {
            for less2 in [false, true] {
                for less3 in [false, true] {
                    for r1 in 0..a1 {
                        for r2 in 0..a2 {
                            for r3 in 0..a3 {
                                for x1 in [0, 1] {
                                    for x2 in [0, 1] {
                                        for x3 in [0, 1] {
                                            if x1 ^ x2 ^ x3 == 1 {
                                                continue;
                                            }
                                            if less1 == false && (n >> i & 1) < x1 {
                                                continue;
                                            }
                                            if less2 == false && (n >> i & 1) < x2 {
                                                continue;
                                            }
                                            if less3 == false && (n >> i & 1) < x3 {
                                                continue;
                                            }
                                            let new_less1 = less1 || (n >> i & 1) > x1;
                                            let new_less2 = less2 || (n >> i & 1) > x2;
                                            let new_less3 = less3 || (n >> i & 1) > x3;
                                            let new_r1 = (r1 + (x1 << i)) % a1;
                                            let new_r2 = (r2 + (x2 << i)) % a2;
                                            let new_r3 = (r3 + (x3 << i)) % a3;
                                            #[rustfmt::skip]
                                            add!(
                                                new_dp[usize::from(new_less1)]
                                                      [usize::from(new_less2)]
                                                      [usize::from(new_less3)]
                                                      [new_r1]
                                                      [new_r2]
                                                      [new_r3],
                                                dp[usize::from(less1)]
                                                  [usize::from(less2)]
                                                  [usize::from(less3)]
                                                  [r1]
                                                  [r2]
                                                  [r3]
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        dp = new_dp;
    }

    let mut ans = 0;
    for less1 in [false, true] {
        for less2 in [false, true] {
            for less3 in [false, true] {
                add!(
                    ans,
                    dp[usize::from(less1)][usize::from(less2)][usize::from(less3)][0][0][0]
                );
            }
        }
    }

    let mut z = 0;

    // x1 = 0, x2 ^ x3 = 0
    add!(z, n / lcm(a2, a3) + 1);
    // x2 = 0, x1 ^ x3 = 0
    add!(z, n / lcm(a1, a3) + 1);
    // x3 = 0, x1 ^ x2 = 0
    add!(z, n / lcm(a1, a2) + 1);

    // x1 = x2 = 0
    sub!(z, 1);
    // x1 = x3 = 0
    sub!(z, 1);
    // x2 = x3 = 0
    sub!(z, 1);

    // x1 = x2 = x3 = 0
    add!(z, 1);

    sub!(ans, z);

    println!("{}", ans);
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

// 13 2 3 5
// 13 = 1101

//  6 = 0110
//  3 = 0011
//  5 = 0101

//  6 = 0110
// 12 = 1100
// 10 = 1010

// 12 = 1100
//  6 = 0110
// 10 = 1010

// 12 = 1100
//  9 = 1001
//  5 = 0101

// (10, 0, 10), (6, 6, 0), (12, 12, 0), (0, 0, 0)

// (0, 0, 0) -> (0, 2, 3) -> (0, 0, 2) -> (0, 2, 4) -> (0, 0, 0)
