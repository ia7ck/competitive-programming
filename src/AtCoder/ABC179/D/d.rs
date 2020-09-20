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

macro_rules! add {
    ($a:expr, $b:expr) => {
        $a = $a + $b;
    };
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let k: usize = rd.get();
    let lr: Vec<(usize, usize)> = (0..k)
        .map(|_| {
            let l: usize = rd.get();
            let r: usize = rd.get();
            (l, r)
        })
        .collect();
    use mint::Mint;
    let mo = 998244353;
    let mut dp = vec![Mint::zero(mo); n + 1];
    dp[1] = Mint::one(mo);
    for i in 1..n {
        add!(dp[i], dp[i - 1]);
        for (l, r) in &lr {
            if i + l <= n {
                add!(dp[i + l], dp[i]);
            }
            if i + r + 1 <= n {
                add!(dp[i + r + 1], Mint::new(mo - dp[i].val(), mo));
            }
        }
    }
    println!("{}", dp[n]);
}

pub struct ProconReader<R: std::io::Read> {
    reader: R,
}

impl<R: std::io::Read> ProconReader<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
    pub fn get<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .reader
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&byte| byte == b' ' || byte == b'\n' || byte == b'\r')
            .take_while(|&byte| byte != b' ' && byte != b'\n' && byte != b'\r')
            .collect::<Vec<_>>();
        std::str::from_utf8(&buf)
            .unwrap()
            .parse()
            .ok()
            .expect("Parse Error.")
    }
}
