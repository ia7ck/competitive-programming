use proconio::{input, marker::Bytes};

use crate::mod_int::ModInt998244353;

fn main() {
    input! {
        n: Bytes,
    };

    type Mint = ModInt998244353;

    let mut less = vec![vec![Mint::new(0); 1 << 10]; 3];
    let mut tight = vec![vec![Mint::new(0); 1 << 10]; 3];
    tight[0][0] = Mint::new(1);
    for d in n {
        let d = usize::from(d - b'0');
        let mut new_less = vec![vec![Mint::new(0); 1 << 10]; 3];
        let mut new_tight = vec![vec![Mint::new(0); 1 << 10]; 3];
        for x in 0..=9 {
            for sum in 0..3 {
                for used in 0..(1 << 10) {
                    let new_sum = (sum + x) % 3;
                    let new_used = if x == 0 && used == 0 {
                        used
                    } else {
                        used | (1 << x)
                    };
                    if x < d {
                        new_less[new_sum][new_used] += tight[sum][used];
                    } else if x == d {
                        new_tight[new_sum][new_used] += tight[sum][used];
                    }
                    new_less[new_sum][new_used] += less[sum][used];
                }
            }
        }

        less = new_less;
        tight = new_tight;
    }

    let mut ans = Mint::new(0);
    for dp in [less, tight] {
        for sum in 0..3 {
            for used in 0_usize..(1 << 10) {
                if used == 0 {
                    continue;
                }
                let mul3 = sum == 0;
                let has3 = used >> 3 & 1 == 1;
                let kind3 = used.count_ones() == 3;
                if [mul3, has3, kind3].iter().filter(|&&cond| cond).count() == 1 {
                    ans += dp[sum][used];
                }
            }
        }
    }

    println!("{}", ans.val());
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
