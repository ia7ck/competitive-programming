type Mint = ModInt1000000007;

macro_rules! add {
    ($a: expr, $b: expr) => {
        $a = $a + $b;
    };
}

fn dfs(
    i: usize,
    p: usize,
    g: &Vec<Vec<(usize, u32)>>,
    size: &mut Vec<Mint>,
    dp1: &mut Vec<Mint>,
    dp2: &mut Vec<Mint>,
) {
    size[i] = Mint::new(1);
    for &(j, w) in &g[i] {
        if j == p {
            continue;
        }
        dfs(j, i, g, size, dp1, dp2);
        add!(size[i], size[j]);
        let w = Mint::new(w);
        add!(dp1[i], dp1[j] + size[j] * w);
        add!(dp2[i], dp2[j] + dp1[j] * w * Mint::new(2) + size[j] * w * w);
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let mut g = vec![vec![]; n];
    for _ in 0..(n - 1) {
        let u: usize = rd.get();
        let v: usize = rd.get();
        let w: u32 = rd.get();
        g[u - 1].push((v - 1, w));
        g[v - 1].push((u - 1, w));
    }
    let mut size = vec![Mint::new(0); n];
    let mut dp1 = vec![Mint::new(0); n];
    let mut dp2 = vec![Mint::new(0); n];
    dfs(0, !0, &g, &mut size, &mut dp1, &mut dp2);
    let mut ans1 = vec![Mint::new(0); n];
    let mut ans2 = vec![Mint::new(0); n];
    ans1[0] = dp1[0];
    ans2[0] = dp2[0];
    use std::collections::VecDeque;
    let mut q = VecDeque::new();
    q.push_back((0, !0));
    while let Some((i, p)) = q.pop_front() {
        for &(j, w) in &g[i] {
            if j == p {
                continue;
            }
            let w = Mint::new(w);
            ans1[j] = ans1[i] + (size[0] - size[j]) * w - size[j] * w;
            ans2[j] = ans2[i] - dp1[j] * w * Mint::new(2) - size[j] * w * w
                + (ans1[i] + (size[0] - size[j]) * w - size[j] * w - dp1[j]) * w * Mint::new(2)
                - (size[0] - size[j]) * w * w;
            q.push_back((j, i));
        }
    }
    let mut tot = Mint::new(0);
    for ans in ans2 {
        add!(tot, ans);
    }
    let ans = tot / Mint::new(2);
    println!("{}", ans.val());
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
