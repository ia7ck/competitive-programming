fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let h: usize = rd.get();
    let w: usize = rd.get();
    let k: usize = rd.get();
    let mut a = vec![vec!['.'; w]; h];
    for _ in 0..k {
        let i: usize = rd.get();
        let j: usize = rd.get();
        let c: char = rd.get();
        a[i - 1][j - 1] = c;
    }

    // up[i][j]: '.' の個数 in {a[0][j], a[1][j], ..., a[i-1][j]}
    let mut up = vec![vec![0; w]; h];
    let mut left = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            if i >= 1 {
                up[i][j] = up[i - 1][j] + if a[i - 1][j] == '.' { 1 } else { 0 };
            }
            if j >= 1 {
                left[i][j] = left[i][j - 1] + if a[i][j - 1] == '.' { 1 } else { 0 };
            }
        }
    }

    macro_rules! add {
        ($a: expr, $b: expr) => {
            $a = $a + $b;
        };
    }

    type Mint = ModInt998244353;
    let zero = Mint::new(0);
    let one = Mint::new(1);
    let two = Mint::new(2);
    let three = Mint::new(3);
    let mut pow3 = vec![one; h.max(w)];
    for i in 1..pow3.len() {
        pow3[i] = pow3[i - 1] * three;
    }
    let mut dp = vec![vec![zero; w]; h];
    dp[0][0] = one;
    for i in 0..h {
        for j in 0..w {
            if i + 1 < h {
                let t = match a[i][j] {
                    'D' | 'X' => one,
                    'R' => zero,
                    '.' => two,
                    _ => unreachable!(),
                };
                add!(dp[i + 1][j], dp[i][j] * t * pow3[left[i + 1][j]]);
            }
            if j + 1 < w {
                let t = match a[i][j] {
                    'R' | 'X' => one,
                    'D' => zero,
                    '.' => two,
                    _ => unreachable!(),
                };
                add!(dp[i][j + 1], dp[i][j] * t * pow3[up[i][j + 1]]);
            }
        }
    }
    let ans = dp[h - 1][w - 1];
    if a[h - 1][w - 1] == '.' {
        println!("{}", (ans * three).val());
    } else {
        println!("{}", ans.val());
    }
}

use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Sub};

pub trait Modulo: Copy + Clone + Debug {
    fn p() -> i64;
}

#[derive(Copy, Clone, Debug)]
pub struct ModInt<M>(i64, PhantomData<M>);

impl<M: Modulo> ModInt<M> {
    pub fn new<T>(x: T) -> Self
    where
        T: Into<i64>,
    {
        let x = x.into();
        if 0 <= x && x < M::p() {
            Self::new_raw(x)
        } else {
            Self::new_raw(x.rem_euclid(M::p()))
        }
    }

    pub fn new_raw(x: i64) -> Self {
        Self(x, PhantomData)
    }

    pub fn val(self) -> i64 {
        self.0
    }

    pub fn mo() -> i64 {
        M::p()
    }

    pub fn pow(self, exp: u64) -> Self {
        let mut res = 1;
        let mut base = self.0;
        let mut exp = exp;
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
        self.pow(Self::mo() as u64 - 2)
    }

    pub fn new_frac(numer: i64, denom: i64) -> Self {
        Self::new(numer) / Self::new(denom)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<M: Modulo> Add for ModInt<M> {
    type Output = ModInt<M>;
    fn add(self, rhs: ModInt<M>) -> Self::Output {
        let x = self.0 + rhs.0;
        debug_assert!(x >= 0);
        if x < Self::mo() {
            Self::new_raw(x)
        } else {
            Self::new_raw(x - Self::mo())
        }
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<M: Modulo> Sub for ModInt<M> {
    type Output = ModInt<M>;
    fn sub(self, rhs: ModInt<M>) -> Self::Output {
        let x = self.0 - rhs.0;
        debug_assert!(x < Self::mo());
        if x >= 0 {
            Self::new_raw(x)
        } else {
            Self::new_raw(x + Self::mo())
        }
    }
}

impl<M: Modulo> Mul for ModInt<M> {
    type Output = ModInt<M>;
    fn mul(self, rhs: ModInt<M>) -> Self::Output {
        Self::new(self.0 * rhs.0)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<M: Modulo> Div for ModInt<M> {
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
    line: String,
    i: usize,
}

impl<R: std::io::BufRead> ProconReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            r: reader,
            line: String::new(),
            i: 0,
        }
    }
    pub fn get<T>(&mut self) -> T
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.skip_blanks();
        assert!(self.i < self.line.len());
        assert_ne!(&self.line[self.i..=self.i], " ");
        let line = &self.line[self.i..];
        let end = line.find(' ').unwrap_or_else(|| line.len());
        let s = &line[..end];
        self.i += end;
        s.parse()
            .unwrap_or_else(|_| panic!("parse error `{}`", self.line))
    }
    fn skip_blanks(&mut self) {
        loop {
            let start = self.line[self.i..].find(|ch| ch != ' ');
            match start {
                Some(j) => {
                    self.i += j;
                    break;
                }
                None => {
                    self.line.clear();
                    self.i = 0;
                    let num_bytes = self.r.read_line(&mut self.line).unwrap();
                    assert!(num_bytes > 0, "reached EOF :(");
                    self.line = self.line.trim_end_matches(&['\r', '\n'][..]).to_string();
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
}
