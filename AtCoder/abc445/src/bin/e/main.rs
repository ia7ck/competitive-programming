use std::collections::HashMap;

use proconio::input;

use crate::{least_prime_factors::least_prime_factors, mod_int::ModInt998244353};

fn main() {
    input! {
        t: usize,
    };

    let lpf = least_prime_factors(10_000_000 + 1);

    for _ in 0..t {
        input! {
            n: usize,
            a: [usize; n],
        };

        solve(n, a, &lpf);
    }
}

fn solve(n: usize, a: Vec<usize>, lpf: &[usize]) {
    let mut p_exp = Vec::new();
    let mut p_exp_top2 = HashMap::<usize, Top2<u32>>::new();
    for &a in &a {
        let mut p_e = Vec::new();
        let mut a = a;
        while a > 1 {
            let p = lpf[a];
            let mut e = 0;
            while a % p == 0 {
                a /= p;
                e += 1;
            }
            p_e.push((p, e));
            p_exp_top2.entry(p).or_insert(Top2::new()).add(e);
        }
        p_exp.push(p_e);
    }

    type Mint = ModInt998244353;

    let mut ans = Vec::new();
    let lcm = p_exp_top2.iter().fold(Mint::new(1), |acc, (&p, top2)| {
        let &exp = top2.get(0).unwrap();
        acc * Mint::from(p).pow(exp)
    });
    for i in 0..n {
        let mut lcm = lcm;
        for &(p, e) in &p_exp[i] {
            let exp = &p_exp_top2[&p];
            let &e0 = exp.get(0).unwrap();
            assert!(e0 >= e);
            if e0 == e {
                let &e1 = exp.get(1).unwrap_or(&0);
                lcm /= Mint::from(p).pow(e - e1);
            }
        }
        ans.push(lcm);
    }

    println!(
        "{}",
        ans.iter()
            .map(|ans| ans.val().to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

#[derive(Debug, Clone)]
struct Top2<T>([Option<T>; 2]);

impl<T> Top2<T>
where
    T: Ord,
{
    fn new() -> Self {
        Self([None, None])
    }

    fn add(&mut self, x: T) {
        let b = self.0[1].take();
        self.0[1] = b.max(Some(x));

        if &self.0[0] < &self.0[1] {
            self.0.swap(0, 1);
        }
    }

    fn get(&self, i: usize) -> Option<&T> {
        assert!(i < 2);

        self.0[i].as_ref()
    }
}

// Bundled
#[rustfmt::skip]
#[allow(unused)]
mod least_prime_factors {
    pub fn least_prime_factors(n: usize) -> Vec<usize> {
        let mut result = vec![0; n];
        #[allow(clippy::needless_range_loop)]
        for i in 2..n {
            result[i] = i;
        }
        for i in 2..n {
            if result[i] == i {
                for j in ((i + i)..n).step_by(i) {
                    if result[j] == j {
                        result[j] = i;
                    }
                }
            }
        }
        result
    }
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
