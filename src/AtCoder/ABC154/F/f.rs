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

#[allow(dead_code)]
mod mint {
    use std::ops::{Add, BitAnd, Div, Mul, Rem, Shr, Sub};

    #[derive(Copy, Clone)]
    pub struct Mint<T> {
        x: T,
        mo: T,
    }
    impl<T> Mint<T>
    where
        T: Copy,
    {
        pub fn new(x: T, mo: T) -> Mint<T> {
            Mint { x, mo }
        }
    }
    impl<T> Mint<T>
    where
        T: Copy,
    {
        pub fn val(&self) -> T {
            self.x
        }
        pub fn mo(&self) -> T {
            self.mo
        }
    }
    impl<T> Add<T> for Mint<T>
    where
        T: Copy,
        T: Add<Output = T>,
        T: Rem<Output = T>,
    {
        type Output = Mint<T>;
        fn add(self, rhs: T) -> Mint<T> {
            Mint::new((self.val() + rhs % self.mo()) % self.mo(), self.mo())
        }
    }
    impl<T> Add<Mint<T>> for Mint<T>
    where
        T: Copy,
        Mint<T>: Add<T, Output = Mint<T>>,
    {
        type Output = Mint<T>;
        fn add(self, rhs: Mint<T>) -> Mint<T> {
            self + rhs.val()
        }
    }
    impl<T> Sub<T> for Mint<T>
    where
        T: Copy,
        T: Add<Output = T>,
        T: Sub<Output = T>,
        T: Rem<Output = T>,
    {
        type Output = Mint<T>;
        fn sub(self, rhs: T) -> Mint<T> {
            Mint::new(
                (self.val() + self.mo() - rhs % self.mo()) % self.mo(),
                self.mo(),
            )
        }
    }
    impl<T> Sub<Mint<T>> for Mint<T>
    where
        T: Copy,
        Mint<T>: Sub<T, Output = Mint<T>>,
    {
        type Output = Mint<T>;
        fn sub(self, rhs: Mint<T>) -> Mint<T> {
            self - rhs.val()
        }
    }
    impl<T> Mul<T> for Mint<T>
    where
        T: Copy,
        T: Mul<Output = T>,
        T: Rem<Output = T>,
    {
        type Output = Mint<T>;
        fn mul(self, rhs: T) -> Mint<T> {
            Mint::new((self.val() * rhs % self.mo()) % self.mo(), self.mo())
        }
    }
    impl<T> Mul<Mint<T>> for Mint<T>
    where
        T: Copy,
        Mint<T>: Mul<T, Output = Mint<T>>,
    {
        type Output = Mint<T>;
        fn mul(self, rhs: Mint<T>) -> Mint<T> {
            self * rhs.val()
        }
    }

    impl<T> Mint<T>
    where
        T: Copy,
        T: Sub<Output = T>,
        T: Div<Output = T>,
        T: PartialOrd,
        T: PartialEq,
        T: BitAnd<Output = T>,
        T: Shr<Output = T>,
        Mint<T>: Mul<Output = Mint<T>>,
    {
        pub fn pow(self, y: T) -> Mint<T> {
            let one = self.mo() / self.mo();
            let zero = self.mo() - self.mo();
            let mut res = Mint::one(self.mo());
            let mut base = self;
            let mut exp = y;
            while exp > zero {
                if (exp & one) == one {
                    res = res * base;
                }
                base = base * base;
                exp = exp >> one;
            }
            res
        }
    }
    impl<T> Div<T> for Mint<T>
    where
        T: Copy,
        T: Sub<Output = T>,
        T: Div<Output = T>,
        T: PartialOrd,
        T: PartialEq,
        T: BitAnd<Output = T>,
        T: Shr<Output = T>,
        Mint<T>: Mul<Output = Mint<T>>,
    {
        type Output = Mint<T>;
        fn div(self, rhs: T) -> Mint<T> {
            let one = self.mo() / self.mo();
            self * Mint::new(rhs, self.mo()).pow(self.mo() - one - one)
        }
    }
    impl<T> Div<Mint<T>> for Mint<T>
    where
        T: Copy,
        Mint<T>: Div<T, Output = Mint<T>>,
    {
        type Output = Mint<T>;
        fn div(self, rhs: Mint<T>) -> Mint<T> {
            self / rhs.val()
        }
    }
    impl<T> Mint<T>
    where
        T: Copy,
        T: Div<Output = T>,
        Mint<T>: Div<Output = Mint<T>>,
    {
        pub fn inv(self) -> Mint<T> {
            Mint::one(self.mo()) / self
        }
    }
    impl<T> std::fmt::Display for Mint<T>
    where
        T: Copy + std::fmt::Display,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.val())
        }
    }
    impl<T> Mint<T>
    where
        T: Copy,
        T: Sub<Output = T>,
    {
        pub fn zero(mo: T) -> Mint<T> {
            Mint { x: mo - mo, mo }
        }
    }
    impl<T> Mint<T>
    where
        T: Copy,
        T: Div<Output = T>,
    {
        pub fn one(mo: T) -> Mint<T> {
            Mint { x: mo / mo, mo }
        }
    }
}

use mint::Mint;

fn main() {
    let r1: usize = read();
    let c1: usize = read();
    let r2: usize = read();
    let c2: usize = read();

    let mo = 1000000007;
    let solve = |r: usize, c: usize| -> Mint<usize> {
        let (fac, fac_inv) = (|n, mo| {
            let mut fac = vec![Mint::new(0, mo); n];
            let mut fac_inv = vec![Mint::new(0, mo); n];
            fac[0] = Mint::new(1, mo);
            for i in 1..n {
                fac[i] = fac[i - 1] * i;
            }
            fac_inv[n - 1] = fac[n - 1].inv();
            for i in (0..n - 1).rev() {
                fac_inv[i] = fac_inv[i + 1] * (i + 1);
            }
            (fac, fac_inv)
        })(r + c + 2, mo);
        let binom = |a: usize, b: usize| {
            if a < b {
                return Mint::new(0, mo);
            }
            fac[a] * fac_inv[b] * fac_inv[a - b]
        };
        let mut ans = Mint::new(0, mo);
        for i in 0..=r {
            ans = ans + binom((i + 1) + c, c);
        }
        ans
    };

    let mut ans = Mint::new(0, mo);
    ans = ans + solve(r2, c2);
    ans = ans - solve(r1 - 1, c2);
    ans = ans - solve(r2, c1 - 1);
    ans = ans + solve(r1 - 1, c1 - 1);
    println!("{}", ans);
}
