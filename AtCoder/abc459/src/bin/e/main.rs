use ac_library::ModInt998244353;
use proconio::{input, marker::Usize1};

type Mint = ModInt998244353;

fn main() {
    input! {
        n: usize,
        p: [Usize1; n - 1],
        c: [usize; n],
        d: [usize; n],
    };

    let mut g = vec![vec![]; n];
    for i in 0..(n - 1) {
        g[p[i]].push(i + 1);
    }

    let mut ans = vec![None; n];
    solve(0, &g, &c, &d, &mut vec![0; n], &mut ans);

    println!("{}", ans[0].map(|ans| ans.val()).unwrap_or(0));
}

fn solve(
    i: usize,
    g: &Vec<Vec<usize>>,
    c: &Vec<usize>,
    d: &Vec<usize>,
    rest: &mut Vec<usize>,
    ans: &mut Vec<Option<Mint>>,
) {
    let mut r_sum = 0;
    let mut prod = Mint::new(1);
    let mut fail = false;
    for &j in &g[i] {
        solve(j, g, c, d, rest, ans);
        r_sum += rest[j];
        match ans[j] {
            Some(ans) => prod *= ans,
            None => fail = true,
        }
    }

    if fail || r_sum + c[i] < d[i] {
        ans[i] = None;
    } else {
        ans[i] = Some(binom(r_sum + c[i], d[i]) * prod);
        rest[i] = r_sum + c[i] - d[i];
    }
}

// sum(k) <= 1e6
fn binom(n: usize, k: usize) -> Mint {
    assert!(n >= k);

    let mut num = Mint::new(1);
    let mut den = Mint::new(1);

    for i in 1..=k {
        num *= n - i + 1;
        den *= i;
    }

    num / den
}

// Bundled
#[rustfmt::skip]
#[allow(unused)]
mod mod_int {

    use std::fmt::Debug;
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

    use ext_gcd::ext_gcd;

    #[derive(Debug, Clone, Copy)]
    pub struct ModInt<const M: i64>(i64);

    impl<const M: i64> ModInt<M> {
        pub fn new(x: i64) -> Self {
            if 0 <= x && x < M {
                Self::new_raw(x)
            } else {
                Self::new_raw(x.rem_euclid(M))
            }
        }

        fn new_raw(x: i64) -> Self {
            debug_assert!(0 <= x && x < M);
            Self(x)
        }

        pub fn val(self) -> i64 {
            self.0
        }

        pub fn modulo() -> i64 {
            M
        }

        pub fn pow(self, exp: u32) -> Self {
            let mut res = 1;
            let mut base = self.0;
            let mut exp = exp;
            while exp > 0 {
                if exp & 1 == 1 {
                    res *= base;
                    res %= M;
                }
                base *= base;
                base %= M;
                exp >>= 1;
            }
            Self::new_raw(res)
        }

        pub fn inv(self) -> Self {
            assert_ne!(self.0, 0, "Don't divide by zero!");
            let (x, _, g) = ext_gcd(self.0, M);
            assert_eq!(g, 1, "{} is not prime!", M);
            Self::new(x)
        }
    }

    impl<const M: i64, T: Into<ModInt<M>>> AddAssign<T> for ModInt<M> {
        fn add_assign(&mut self, rhs: T) {
            self.0 += rhs.into().0;
            debug_assert!(0 <= self.0 && self.0 <= (M - 1) * 2);
            if self.0 >= M {
                self.0 -= M;
            }
        }
    }

    impl<const M: i64, T: Into<ModInt<M>>> Add<T> for ModInt<M> {
        type Output = ModInt<M>;
        fn add(self, rhs: T) -> Self::Output {
            let mut result = self;
            result += rhs.into();
            result
        }
    }

    impl<const M: i64, T: Into<ModInt<M>>> SubAssign<T> for ModInt<M> {
        fn sub_assign(&mut self, rhs: T) {
            self.0 -= rhs.into().0;
            debug_assert!(-(M - 1) <= self.0 && self.0 < M);
            if self.0 < 0 {
                self.0 += M;
            }
        }
    }

    impl<const M: i64, T: Into<ModInt<M>>> Sub<T> for ModInt<M> {
        type Output = ModInt<M>;
        fn sub(self, rhs: T) -> Self::Output {
            let mut result = self;
            result -= rhs.into();
            result
        }
    }

    impl<const M: i64, T: Into<ModInt<M>>> MulAssign<T> for ModInt<M> {
        fn mul_assign(&mut self, rhs: T) {
            self.0 *= rhs.into().0;
            if self.0 >= M {
                self.0 %= M;
            }
        }
    }

    impl<const M: i64, T: Into<ModInt<M>>> Mul<T> for ModInt<M> {
        type Output = ModInt<M>;
        fn mul(self, rhs: T) -> Self::Output {
            let mut result = self;
            result *= rhs.into();
            result
        }
    }

    #[allow(clippy::suspicious_op_assign_impl)]
    impl<const M: i64, T: Into<ModInt<M>>> DivAssign<T> for ModInt<M> {
        fn div_assign(&mut self, rhs: T) {
            *self *= rhs.into().inv();
        }
    }

    impl<const M: i64, T: Into<ModInt<M>>> Div<T> for ModInt<M> {
        type Output = ModInt<M>;
        fn div(self, rhs: T) -> Self::Output {
            let mut result = self;
            result /= rhs.into();
            result
        }
    }

    macro_rules! impl_from_int {
        ($($t:ty),+) => {
            $(
                impl<const M: i64> From<$t> for ModInt<M> {
                    fn from(x: $t) -> Self {
                        Self::new(i64::from(x))
                    }
                }
            )+
        };
    }

    impl_from_int!(i8, i16, i32, i64, u8, u16, u32);

    macro_rules! impl_from_large_int {
        ($($t:ty),+) => {
            $(
                impl<const M: i64> From<$t> for ModInt<M> {
                    fn from(x: $t) -> Self {
                        Self::new((x % (M as $t)) as i64)
                    }
                }
            )+
        };
    }

    impl_from_large_int!(u64, usize, isize);

    pub type ModInt1000000007 = ModInt<1_000_000_007>;
    pub type ModInt998244353 = ModInt<998_244_353>;

    mod ext_gcd {
        #[allow(clippy::many_single_char_names)]
        pub fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
            if b == 0 {
                if a == 0 { (0, 0, 0) } else { (1, 0, a) }
            } else {
                let (q, r) = (a / b, a % b);
                let (s, t, g) = ext_gcd(b, r);
                (t, s - q * t, g)
            }
        }
    }
}

