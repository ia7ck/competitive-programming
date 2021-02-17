type Mint = ModInt998244353;
macro_rules! mint {
    ($x: expr) => {
        Mint::new($x as i64)
    };
}
macro_rules! add {
    ($a: ident, $b: expr) => {
        $a = $a + $b;
    };
}
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Black {
    Zero,
    One,
    TwoOrMore,
}
fn solve(
    a: usize,
    b: usize,
    c: usize,
    d: usize,
    black: Black,
    memo: &mut Vec<Vec<Vec<Mint>>>,
    seen: &mut Vec<Vec<Vec<bool>>>,
) -> Mint {
    use Black::{One, TwoOrMore, Zero};
    if a == c && b == d {
        return match black {
            Zero => mint!(1),
            _ => mint!(0),
        };
    }
    if seen[black as usize][c][d] {
        return memo[black as usize][c][d];
    }
    seen[black as usize][c][d] = true;
    let mut ans = mint!(0);
    match black {
        Zero => {
            if b < d {
                add!(ans, solve(a, b, c, d - 1, black, memo, seen) * mint!(c - 1));
            }
        }
        One => {
            if a < c {
                // 上の行を消す
                add!(ans, solve(a, b, c - 1, d, Zero, memo, seen));
                add!(ans, solve(a, b, c - 1, d, One, memo, seen));
                add!(ans, solve(a, b, c - 1, d, TwoOrMore, memo, seen));
                ans = ans * mint!(d);
            } else {
                // 右の列を消す
                assert!(b < d);
                add!(ans, solve(a, b, c, d - 1, Zero, memo, seen));
                add!(ans, solve(a, b, c, d - 1, One, memo, seen) * mint!(c - 1));
            }
        }
        TwoOrMore => {
            // 右の列を消す
            if b < d {
                // 右上以外のどれかが黒
                add!(
                    ans,
                    solve(a, b, c, d - 1, TwoOrMore, memo, seen) * mint!(c - 1)
                );
                // 右上が黒・上の列はそれ以外にも黒が2つ以上
                add!(ans, solve(a, b, c, d - 1, TwoOrMore, memo, seen));
                // 右上が黒・上の列はそれ以外に黒がちょうど1つ
                add!(ans, solve(a, b, c, d - 1, One, memo, seen));
            }
        }
    }
    // println!("{} {} {:?} {}", c, d, black, ans.val());
    memo[black as usize][c][d] = ans;
    ans
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let a: usize = rd.get();
    let b: usize = rd.get();
    let c: usize = rd.get();
    let d: usize = rd.get();

    use Black::{One, TwoOrMore, Zero};
    let mut memo = vec![vec![vec![mint!(0); d + 1]; c + 1]; 3];
    let mut seen = vec![vec![vec![false; d + 1]; c + 1]; 3];
    let mut ans = mint!(0);
    add!(ans, solve(a, b, c, d, Zero, &mut memo, &mut seen));
    add!(ans, solve(a, b, c, d, One, &mut memo, &mut seen));
    add!(ans, solve(a, b, c, d, TwoOrMore, &mut memo, &mut seen));
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
