fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let m: usize = rd.get();

    type Mint = ModInt998244353;
    let zero = Mint::new(0);
    let one = Mint::new(1);
    let two = Mint::new(2);
    let mut pow = vec![vec![zero; n + 1]; m + 1];
    for i in 0..=m {
        pow[i][0] = one;
        for j in 0..n {
            pow[i][j + 1] = pow[i][j] * Mint::new(i);
        }
    }

    let mut ans = Mint::new(0);
    for h in 1..=m {
        for w in 1..=n {
            let c = pow[m - h + 1][w] - pow[m - h][w]; // 最小値が h
            let c = if w == n {
                c
            } else {
                // 左寄せ or 右寄せ
                let d = c * Mint::new(h - 1) * pow[m][n - w - 1] * two;
                if w == n - 1 {
                    d
                } else {
                    assert!(w <= n - 2);
                    d + c
                        * Mint::new(h - 1)
                        * Mint::new(h - 1)
                        * pow[m][n - w - 2]
                        * Mint::new(n - w - 1)
                }
            };
            ans = ans + c;
        }
    }
    println!("{}", ans.val());
}

pub trait Modulo: Copy + Clone + std::fmt::Debug {
    fn p() -> i64;
}

#[derive(Copy, Clone, std::fmt::Debug)]
pub struct ModInt<M>(i64, std::marker::PhantomData<M>);

impl<M: Modulo> ModInt<M> {
    pub fn new<T>(x: T) -> Self
    where
        T: std::convert::TryInto<i64>,
        <T as std::convert::TryInto<i64>>::Error: std::fmt::Debug,
    {
        let x = x.try_into().unwrap();
        if 0 <= x && x < M::p() {
            Self::new_raw(x)
        } else {
            Self::new_raw(x.rem_euclid(M::p()))
        }
    }

    fn new_raw(x: i64) -> Self {
        debug_assert!(0 <= x && x < M::p());
        Self(x, std::marker::PhantomData)
    }

    pub fn val(self) -> i64 {
        self.0
    }

    pub fn mo() -> i64 {
        M::p()
    }

    pub fn pow<T>(self, exp: T) -> Self
    where
        T: std::convert::TryInto<u64>,
        <T as std::convert::TryInto<u64>>::Error: std::fmt::Debug,
    {
        let mut res = 1;
        let mut base = self.0;
        let mut exp = exp.try_into().unwrap();
        let mo = Self::mo();
        while exp > 0 {
            if exp & 1 == 1 {
                res *= base;
                res %= mo;
            }
            base *= base;
            base %= mo;
            exp >>= 1;
        }
        Self::new_raw(res)
    }

    pub fn inv(self) -> Self {
        assert_ne!(self.0, 0, "Don't divide by zero!");
        self.pow(Self::mo() - 2)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<M: Modulo> std::ops::Add for ModInt<M> {
    type Output = ModInt<M>;
    fn add(self, rhs: ModInt<M>) -> Self::Output {
        let x = self.0 + rhs.0;
        debug_assert!(0 <= x && x <= (Self::mo() - 1) * 2);
        if x < Self::mo() {
            Self::new_raw(x)
        } else {
            Self::new_raw(x - Self::mo())
        }
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<M: Modulo> std::ops::Sub for ModInt<M> {
    type Output = ModInt<M>;
    fn sub(self, rhs: ModInt<M>) -> Self::Output {
        let x = self.0 - rhs.0;
        debug_assert!(-(Self::mo() - 1) <= x && x < Self::mo());
        if x >= 0 {
            Self::new_raw(x)
        } else {
            Self::new_raw(x + Self::mo())
        }
    }
}

impl<M: Modulo> std::ops::Mul for ModInt<M> {
    type Output = ModInt<M>;
    fn mul(self, rhs: ModInt<M>) -> Self::Output {
        Self::new(self.0 * rhs.0)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<M: Modulo> std::ops::Div for ModInt<M> {
    type Output = ModInt<M>;
    fn div(self, rhs: ModInt<M>) -> Self::Output {
        self * rhs.inv()
    }
}

#[macro_export]
macro_rules! define_mod_int_p {
    ($mod: ident, $mod_int: ident, $p: expr) => {
        #[derive(Clone, Copy, Debug)]
        pub struct $mod;
        impl Modulo for $mod {
            fn p() -> i64 {
                $p
            }
        }
        pub type $mod_int = ModInt<$mod>;
    };
}
define_mod_int_p!(Mod1000000007, ModInt1000000007, 1_000_000_000 + 7);
define_mod_int_p!(Mod998244353, ModInt998244353, 998_244_353);

pub struct ProconReader<R> {
    r: R,
    l: String,
    i: usize,
}

impl<R: std::io::BufRead> ProconReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            r: reader,
            l: String::new(),
            i: 0,
        }
    }
    pub fn get<T>(&mut self) -> T
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.skip_blanks();
        assert!(self.i < self.l.len()); // remain some character
        assert_ne!(&self.l[self.i..=self.i], " ");
        let rest = &self.l[self.i..];
        let len = rest.find(' ').unwrap_or_else(|| rest.len());
        // parse self.l[self.i..(self.i + len)]
        let val = rest[..len]
            .parse()
            .unwrap_or_else(|e| panic!("{:?}, attempt to read `{}`", e, rest));
        self.i += len;
        val
    }
    fn skip_blanks(&mut self) {
        loop {
            match self.l[self.i..].find(|ch| ch != ' ') {
                Some(j) => {
                    self.i += j;
                    break;
                }
                None => {
                    let mut buf = String::new();
                    let num_bytes = self
                        .r
                        .read_line(&mut buf)
                        .unwrap_or_else(|_| panic!("invalid UTF-8"));
                    assert!(num_bytes > 0, "reached EOF :(");
                    self.l = buf
                        .trim_end_matches('\n')
                        .trim_end_matches('\r')
                        .to_string();
                    self.i = 0;
                }
            }
        }
    }
    pub fn get_vec<T>(&mut self, n: usize) -> Vec<T>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        (0..n).map(|_| self.get()).collect()
    }
    pub fn get_chars(&mut self) -> Vec<char> {
        self.get::<String>().chars().collect()
    }
}
