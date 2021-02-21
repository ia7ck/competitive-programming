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

struct LazySegmentTree<T, F, U, G, H> {
    n: usize,
    dat: Vec<T>,
    e_t: T,
    multiply: F,
    laz: Vec<U>,
    e_u: U,
    composite: G,
    apply: H,
}

impl<T, F, U, G, H> LazySegmentTree<T, F, U, G, H>
where
    T: Copy,
    F: Fn(T, T) -> T,
    U: Copy,
    G: Fn(U, U) -> U,
    H: Fn(U, T) -> T,
{
    fn new(a: &Vec<T>, e_t: T, multiply: F, e_u: U, composite: G, apply: H) -> Self {
        let len = a.len();
        let n = len.next_power_of_two();
        let mut dat = vec![e_t; n * 2 - 1];
        for i in 0..len {
            dat[i + n - 1] = a[i];
        }
        for i in (0..(n - 1)).rev() {
            dat[i] = (multiply)(dat[i * 2 + 1], dat[i * 2 + 2]);
        }
        Self {
            n,
            dat,
            e_t,
            multiply,
            laz: vec![e_u; n * 2 - 1],
            e_u,
            composite,
            apply,
        }
    }
    fn update_node(&mut self, i: usize, u: U) {
        self.dat[i] = (self.apply)(u, self.dat[i]);
        self.laz[i] = (self.composite)(u, self.laz[i]);
    }
    fn update_range(
        &mut self,
        range: &std::ops::Range<usize>,
        i: usize,
        i_range: std::ops::Range<usize>,
        u: U,
    ) {
        if range.end <= i_range.start || i_range.end <= range.start {
            return;
        }
        if range.start <= i_range.start && i_range.end <= range.end {
            self.update_node(i, u);
            return;
        }
        let left_child = i * 2 + 1;
        let right_child = i * 2 + 2;
        self.update_node(left_child, self.laz[i]);
        self.update_node(right_child, self.laz[i]);
        self.laz[i] = self.e_u;
        let m = (i_range.start + i_range.end) / 2;
        self.update_range(range, left_child, i_range.start..m, u);
        self.update_range(range, right_child, m..i_range.end, u);
        self.dat[i] = (self.multiply)(self.dat[left_child], self.dat[right_child]);
    }
    fn update(&mut self, range: std::ops::Range<usize>, u: U) {
        self.update_range(&range, 0, 0..self.n, u);
    }
    fn _fold(
        &mut self,
        range: &std::ops::Range<usize>,
        i: usize,
        i_range: std::ops::Range<usize>,
    ) -> T {
        if range.end <= i_range.start || i_range.end <= range.start {
            return self.e_t;
        }
        if range.start <= i_range.start && i_range.end <= range.end {
            return self.dat[i];
        }
        let m = (i_range.start + i_range.end) / 2;
        let left_result = self._fold(range, i * 2 + 1, i_range.start..m);
        let right_result = self._fold(range, i * 2 + 2, m..i_range.end);
        (self.apply)(self.laz[i], (self.multiply)(left_result, right_result))
    }
    fn fold(&mut self, range: std::ops::Range<usize>) -> T {
        self._fold(&range, 0, 0..self.n)
    }
}

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

use mint::Mint;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let q: usize = rd.get();
    let a: Vec<i64> = (0..n).map(|_| rd.get()).collect();
    let mo = 998244353;
    let mut seg = LazySegmentTree::new(
        &a.iter().map(|&x| (Mint::new(x, mo), 1)).collect(),
        (Mint::zero(mo), 0),
        |a, b| (a.0 + b.0, a.1 + b.1),
        (Mint::one(mo), Mint::zero(mo)),
        |p, q| (p.0 * q.0, p.0 * q.1 + p.1),
        |p, a| (p.0 * a.0 + p.1 * a.1, a.1),
    );
    for _ in 0..q {
        let t: usize = rd.get();
        match t {
            0 => {
                let l: usize = rd.get();
                let r: usize = rd.get();
                let b: i64 = rd.get();
                let c: i64 = rd.get();
                seg.update(l..r, (Mint::new(b, mo), Mint::new(c, mo)));
            }
            1 => {
                let l: usize = rd.get();
                let r: usize = rd.get();
                let (sum, _) = seg.fold(l..r);
                println!("{}", sum);
            }
            _ => unreachable!(),
        }
    }
}
