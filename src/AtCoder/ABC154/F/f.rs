use std::io::Read;

fn read<T: std::str::FromStr>() -> T {
    let token: String = std::io::stdin()
        .bytes()
        .map(|c| c.ok().unwrap() as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().unwrap()
}

mod mint {
    use std::ops::{Add, Div, Mul, Sub};

    type Int = i64;
    pub const MOD: Int = 1_000_000_000 + 7;
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
        pub fn pow(x: Mint, y: Int) -> Mint {
            assert!(y >= 0);
            let mut res = Mint::new(1);
            let mut base = x;
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
            Mint::pow(self, MOD - 2)
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

struct Cmb {
    fac: Vec<Mint>,
    fac_inv: Vec<Mint>,
}

impl Cmb {
    pub fn new(n: usize) -> Cmb {
        let mut fac = vec![Mint::new(0); n];
        let mut fac_inv = vec![Mint::new(0); n];
        fac[0] = Mint::new(1);
        for i in 1..n {
            fac[i] = fac[i - 1] * (i as i64);
        }
        fac_inv[n - 1] = fac[n - 1].inv();
        for i in (0..n - 1).rev() {
            fac_inv[i] = fac_inv[i + 1] * ((i + 1) as i64)
        }
        Cmb { fac, fac_inv }
    }
    pub fn binom(&self, a: usize, b: usize) -> Mint {
        if a < b {
            return Mint::new(0);
        }
        self.fac[a] * self.fac_inv[b] * self.fac_inv[a - b]
    }
}

fn main() {
    let r1: usize = read();
    let c1: usize = read();
    let r2: usize = read();
    let c2: usize = read();

    fn solve(r: usize, c: usize) -> Mint {
        let cmb = Cmb::new(r + c + 2);
        let mut ans = Mint::new(0);
        for i in 0..=r {
            ans = ans + cmb.binom((i + 1) + c, c);
        }
        ans
    }
    let mut ans = Mint::new(0);
    ans = ans + solve(r2, c2);
    ans = ans - solve(r1 - 1, c2);
    ans = ans - solve(r2, c1 - 1);
    ans = ans + solve(r1 - 1, c1 - 1);
    println!("{}", ans);
}
