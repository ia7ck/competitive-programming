extern crate proconio;
use proconio::input;

mod mint {
    use std::ops::{Add, AddAssign, Div, Mul, Sub};

    type Int = i64;
    pub const MOD: Int = 998244353;
    #[derive(Clone, Copy)]
    pub struct Mint {
        x: Int,
    }
    impl Mint {
        pub fn new(x: Int) -> Mint {
            Mint {
                x: (x % MOD + MOD) % MOD,
            }
        }
        pub fn val(&self) -> Int {
            self.x
        }
    }
    impl Add<Int> for Mint {
        type Output = Mint;
        fn add(self, rhs: Int) -> Mint {
            Mint::new(self.val() + (rhs % MOD))
        }
    }
    impl Add for Mint {
        type Output = Mint;
        fn add(self, rhs: Mint) -> Mint {
            self + rhs.val()
        }
    }
    impl AddAssign<Int> for Mint {
        fn add_assign(&mut self, other: Int) {
            *self = *self + other;
        }
    }
    impl AddAssign for Mint {
        fn add_assign(&mut self, other: Mint) {
            *self = *self + other;
        }
    }
    impl Sub<Int> for Mint {
        type Output = Mint;
        fn sub(self, rhs: Int) -> Mint {
            Mint::new(self.val() - (rhs % MOD))
        }
    }
    impl Sub for Mint {
        type Output = Mint;
        fn sub(self, rhs: Mint) -> Mint {
            self - rhs.val()
        }
    }
    impl Mul<Int> for Mint {
        type Output = Mint;
        fn mul(self, rhs: Int) -> Mint {
            Mint::new(self.val() * (rhs % MOD))
        }
    }
    impl Mul for Mint {
        type Output = Mint;
        fn mul(self, rhs: Mint) -> Mint {
            self * rhs.val()
        }
    }
    impl Mint {
        pub fn pow(self, y: Int) -> Mint {
            assert!(y >= 0);
            let mut res = Mint::new(1);
            let mut base = self;
            let mut exp = y;
            while exp > 0 {
                if (exp & 1) == 1 {
                    res = res * base;
                }
                base = base * base;
                exp = exp / 2;
            }
            res
        }
        pub fn inv(self) -> Mint {
            self.pow(MOD - 2)
        }
    }
    impl Div<Int> for Mint {
        type Output = Mint;
        fn div(self, rhs: Int) -> Mint {
            self * Mint::new(rhs).inv()
        }
    }
    impl Div for Mint {
        type Output = Mint;
        fn div(self, rhs: Mint) -> Mint {
            self * rhs.val()
        }
    }
    impl std::fmt::Display for Mint {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.val())
        }
    }
}

use mint::Mint;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }
    let mut dp = vec![vec![vec![Mint::new(0); 3]; s + 1]; n + 1];
    dp[0][0][0] = Mint::new(1);
    for i in 0..n {
        for j in 0..=s {
            let x = dp[i][j][0];
            dp[i + 1][j][0] += x;
            if j + a[i] <= s {
                dp[i + 1][j + a[i]][1] += x * ((i + 1) as i64);
                dp[i + 1][j + a[i]][2] += x * ((i + 1) as i64) * ((n - i) as i64);
            }

            let y = dp[i][j][1];
            dp[i + 1][j][1] += y;
            if j + a[i] <= s {
                dp[i + 1][j + a[i]][1] += y;
                dp[i + 1][j + a[i]][2] += y * ((n - i) as i64);
            }

            let z = dp[i][j][2];
            dp[i + 1][j][2] += z;
        }
    }
    println!("{}", dp[n][s][2]);
}
