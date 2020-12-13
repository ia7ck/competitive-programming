fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let x: i32 = rd.get();
    let s: String = rd.get();
    let s: Vec<char> = s.chars().collect();

    let x = x.abs() as usize;
    let a = s.iter().filter(|&&c| c == 'S').count();
    let b = s.iter().filter(|&&c| c != 'S').count();

    type Mint = ModInt1000000007;
    let binom = make_binom_func(n + 1, Mod1000000007::p());
    let binom_m = |n, k| Mint::new(binom(n, k));

    let mut cum = vec![Mint::new(0); b + 1];
    for i in 0..=b {
        cum[i] = binom_m(b, i) * Mint::new(2).pow((b - i) as u64);
        if i >= 1 {
            cum[i] = cum[i] + cum[i - 1];
        }
    }

    let mut ans = Mint::new(0);
    for i in 0..=a {
        if i % 2 != x % 2 {
            continue;
        }
        let y = x.checked_sub(i * 3).unwrap_or(0);
        let y = (y + 6 - 1) / 6;
        if y <= b {
            let s = if y >= 1 { cum[b] - cum[y - 1] } else { cum[b] };
            ans = ans + binom_m(a, i) * Mint::new(2).pow((a - i) as u64) * s;
        }
    }
    println!("{}", ans.val());
}

pub fn make_binom_func(len: usize, mo: i64) -> impl Fn(usize, usize) -> i64 {
    let mut fac = vec![0; len];
    let mut inv = vec![0; len];
    let mut inv_fac = vec![0; len];
    fac[0] = 1;
    fac[1] = 1;
    inv[1] = 1;
    inv_fac[0] = 1;
    inv_fac[1] = 1;
    for i in 2..len {
        fac[i] = fac[i - 1] * (i as i64) % mo;
        inv[i] = (-inv[(mo as usize) % i] * (mo / (i as i64))).rem_euclid(mo);
        inv_fac[i] = inv_fac[i - 1] * inv[i] % mo;
    }
    move |n: usize, k: usize| -> i64 {
        if n < k {
            return 0;
        }
        ((fac[n] * inv_fac[k]) % mo * inv_fac[n - k]) % mo
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
    pub fn new(x: i64) -> Self {
        Self(x.rem_euclid(M::p()), PhantomData)
    }
    pub fn val(self) -> i64 {
        self.0
    }
    pub fn mo(self) -> i64 {
        M::p()
    }
    pub fn pow(self, exp: u64) -> Self {
        let mut res = Self::new(1);
        let mut base = self;
        let mut exp = exp;
        while exp > 0 {
            if exp & 1 == 1 {
                res = res * base;
            }
            base = base * base;
            exp >>= 1;
        }
        res
    }
    pub fn inv(self) -> Self {
        assert_ne!(self.0, 0, "Don't divide by zero!");
        self.pow(M::p() as u64 - 2)
    }
    pub fn new_frac(numer: i64, denom: i64) -> Self {
        Self::new(numer) / Self::new(denom)
    }
}

impl<M: Modulo> Add for ModInt<M> {
    type Output = ModInt<M>;
    fn add(self, rhs: ModInt<M>) -> Self::Output {
        Self((self.0 + rhs.0) % M::p(), PhantomData)
    }
}

impl<M: Modulo> Sub for ModInt<M> {
    type Output = ModInt<M>;
    fn sub(self, rhs: ModInt<M>) -> Self::Output {
        Self((self.0 - rhs.0).rem_euclid(M::p()), PhantomData)
    }
}

impl<M: Modulo> Mul for ModInt<M> {
    type Output = ModInt<M>;
    fn mul(self, rhs: ModInt<M>) -> Self::Output {
        Self((self.0 * rhs.0) % M::p(), PhantomData)
    }
}

impl<M: Modulo> Div for ModInt<M> {
    type Output = ModInt<M>;
    fn div(self, rhs: ModInt<M>) -> Self::Output {
        self * rhs.inv()
    }
}

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
define_mod_int_p!(Mod1000000007, ModInt1000000007, 1000000007);
define_mod_int_p!(Mod998244353, ModInt998244353, 998244353);

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
