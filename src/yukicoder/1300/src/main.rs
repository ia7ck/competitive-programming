fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let a: Vec<usize> = (0..n).map(|_| { rd.get() }).collect();

    let mo: usize = 998244353;
    use mint::Mint;

    let mut ans = Mint::zero(mo);
    let mut left_cnt = vec![0; n]; // 0..j の範囲で a[j] より大きい数の個数
    let mut left_sum = vec![Mint::zero(mo); n]; // 0..j の範囲で a[j] より大きい数の総和
    let mut right_cnt = vec![0; n]; // (j+1)..n の範囲で a[j] より小さい数の個数
    let mut right_sum = vec![Mint::zero(mo); n]; // (j+1)..n の範囲で a[j] より小さい数の総和
    {
        let mut ai = a.iter().zip(0..n).map(|(&x, i)| (x, i)).collect::<Vec<_>>();
        ai.sort_by(|(x, i), (y, j)| {
            if x == y {
                i.cmp(j)
            } else {
                x.cmp(y)
            }
        });
        let mut seg = SegmentTree::new(n, 0usize, |x, y| x + y);
        let mut teg = SegmentTree::new(n, Mint::zero(mo), |x, y| x + y);
        for (x, i) in ai {
            right_cnt[i] = seg.fold((i + 1)..n);
            right_sum[i] = teg.fold((i + 1)..n);
            seg.update(i, 1);
            teg.update(i, Mint::new(x, mo));
        }
    }
    {
        let mut ai = a.iter().zip(0..n).map(|(&x, i)| (x, i)).collect::<Vec<_>>();
        ai.sort_by(|(x, i), (y, j)| {
            if x == y {
                j.cmp(i)
            } else {
                y.cmp(x)
            }
        });
        let mut seg = SegmentTree::new(n, 0usize, |x, y| x + y);
        let mut teg = SegmentTree::new(n, Mint::zero(mo), |x, y| x + y);
        for (x, i) in ai {
            left_cnt[i] = seg.fold(0..i);
            left_sum[i] = teg.fold(0..i);
            seg.update(i, 1);
            teg.update(i, Mint::new(x, mo));
        }
    }
    macro_rules! add {
        ($a:expr, $b:expr) => {
            $a = $a + $b;
        }
    }
    for j in 0..n {
        let l_c = left_cnt[j];
        let r_c = right_cnt[j];
        let l_s = left_sum[j];
        let r_s = right_sum[j];
        add!(ans, a[j] * l_c * r_c  % mo);
        add!(ans, l_s * r_c);
        add!(ans, r_s * l_c);
    }
    println!("{}", ans);
}

struct SegmentTree<T, F> {
    n: usize,
    dat: Vec<T>,
    e: T,
    multiply: F,
}

#[allow(dead_code)]
impl<T, F> SegmentTree<T, F>
    where
        T: Clone + Copy,
        F: Fn(T, T) -> T,
{
    fn new(n: usize, e: T, multiply: F) -> Self {
        let n = n.next_power_of_two();
        Self {
            n,
            dat: vec![e; n * 2 - 1],
            e,
            multiply,
        }
    }
    fn get(&self, i: usize) -> T {
        self.dat[i + self.n - 1]
    }
    fn update(&mut self, i: usize, x: T) {
        let mut k = i + self.n - 1;
        self.dat[k] = x;
        while k > 0 {
            k = (k - 1) / 2;
            self.dat[k] = (self.multiply)(self.dat[k * 2 + 1], self.dat[k * 2 + 2]);
        }
    }
    fn fold(&self, range: std::ops::Range<usize>) -> T {
        self._fold(&range, 0, 0..self.n)
    }
    fn _fold(
        &self,
        range: &std::ops::Range<usize>,
        i: usize,
        i_range: std::ops::Range<usize>,
    ) -> T {
        if range.end <= i_range.start || i_range.end <= range.start {
            return self.e;
        }
        if range.start <= i_range.start && i_range.end <= range.end {
            return self.dat[i];
        }
        let m = (i_range.start + i_range.end) / 2;
        (self.multiply)(
            self._fold(range, i * 2 + 1, i_range.start..m),
            self._fold(range, i * 2 + 2, m..i_range.end),
        )
    }
}

#[allow(dead_code)]
mod mint {
    use std::ops::{Add, BitAnd, Div, Mul, Rem, Shr, Sub};

    #[derive(Copy, Clone, Debug)]
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
            T: Add<Output=T>,
            T: Rem<Output=T>,
    {
        type Output = Mint<T>;
        fn add(self, rhs: T) -> Mint<T> {
            Mint::new((self.val() + rhs % self.mo()) % self.mo(), self.mo())
        }
    }

    impl<T> Add<Mint<T>> for Mint<T>
        where
            T: Copy,
            Mint<T>: Add<T, Output=Mint<T>>,
    {
        type Output = Mint<T>;
        fn add(self, rhs: Mint<T>) -> Mint<T> {
            self + rhs.val()
        }
    }

    impl<T> Sub<T> for Mint<T>
        where
            T: Copy,
            T: Add<Output=T>,
            T: Sub<Output=T>,
            T: Rem<Output=T>,
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
            Mint<T>: Sub<T, Output=Mint<T>>,
    {
        type Output = Mint<T>;
        fn sub(self, rhs: Mint<T>) -> Mint<T> {
            self - rhs.val()
        }
    }

    impl<T> Mul<T> for Mint<T>
        where
            T: Copy,
            T: Mul<Output=T>,
            T: Rem<Output=T>,
    {
        type Output = Mint<T>;
        fn mul(self, rhs: T) -> Mint<T> {
            Mint::new((self.val() * rhs % self.mo()) % self.mo(), self.mo())
        }
    }

    impl<T> Mul<Mint<T>> for Mint<T>
        where
            T: Copy,
            Mint<T>: Mul<T, Output=Mint<T>>,
    {
        type Output = Mint<T>;
        fn mul(self, rhs: Mint<T>) -> Mint<T> {
            self * rhs.val()
        }
    }

    impl<T> Mint<T>
        where
            T: Copy,
            T: Sub<Output=T>,
            T: Div<Output=T>,
            T: PartialOrd,
            T: PartialEq,
            T: BitAnd<Output=T>,
            T: Shr<Output=T>,
            Mint<T>: Mul<Output=Mint<T>>,
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
            T: Sub<Output=T>,
            T: Div<Output=T>,
            T: PartialOrd,
            T: PartialEq,
            T: BitAnd<Output=T>,
            T: Shr<Output=T>,
            Mint<T>: Mul<Output=Mint<T>>,
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
            Mint<T>: Div<T, Output=Mint<T>>,
    {
        type Output = Mint<T>;
        fn div(self, rhs: Mint<T>) -> Mint<T> {
            self / rhs.val()
        }
    }

    impl<T> Mint<T>
        where
            T: Copy,
            T: Div<Output=T>,
            Mint<T>: Div<Output=Mint<T>>,
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
            T: Sub<Output=T>,
    {
        pub fn zero(mo: T) -> Mint<T> {
            Mint { x: mo - mo, mo }
        }
    }

    impl<T> Mint<T>
        where
            T: Copy,
            T: Div<Output=T>,
    {
        pub fn one(mo: T) -> Mint<T> {
            Mint { x: mo / mo, mo }
        }
    }
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
