fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let q: usize = rd.get();

    let mo = 998244353;
    use mint::Mint;

    // 0, 1, 11, 111, 1111, ...
    let mut ones = vec![Mint::zero(mo); n + 1];
    for i in 1..=n {
        ones[i] = ones[i - 1] * 10 + 1;
    }
    // 1, 10, 100, 1000, ...
    let mut pows = vec![Mint::one(mo); n + 1];
    for i in 1..=n {
        pows[i] = pows[i - 1] * 10;
    }

    let id = 123;
    let mut seg = LazySegmentTree::new(
        &(vec![(Mint::one(mo), 1); n]),
        (Mint::zero(mo), 0),
        |(a, a_len), (b, b_len)| (a * pows[b_len] + b, a_len + b_len),
        id,
        |f, g| {
            if f == id {
                g
            } else {
                f
            }
        },
        |f, (a, len)| {
            if f == id {
                (a, len)
            } else {
                (ones[len] * f, len)
            }
        },
    );
    for _ in 0..q {
        let l: usize = rd.get();
        let r: usize = rd.get();
        let d: usize = rd.get();
        seg.update((l - 1)..r, d);
        // println!("{:?}", (0..n).map(|i| seg.get(i)).collect::<Vec<_>>());
        println!("{}", seg.fold(0..n).0);
    }
}

struct LazySegmentTree<T, Multiply, F, Composite, Apply> {
    n: usize,
    dat: Vec<T>,
    e: T,
    multiply: Multiply,
    laz: Vec<F>,
    id: F,
    composite: Composite,
    apply: Apply,
}

impl<T, Multiply, F, Composite, Apply> LazySegmentTree<T, Multiply, F, Composite, Apply>
where
    T: Copy + std::fmt::Debug,
    Multiply: Fn(T, T) -> T,
    F: Copy + std::fmt::Debug,
    Composite: Fn(F, F) -> F,
    Apply: Fn(F, T) -> T,
{
    fn new(
        a: &Vec<T>,
        e: T,
        multiply: Multiply,
        id: F,
        composite: Composite,
        apply: Apply,
    ) -> Self {
        let len = a.len();
        let n = len.next_power_of_two();
        let mut dat = vec![e; n * 2 - 1];
        for i in 0..len {
            dat[i + n - 1] = a[i];
        }
        for i in (0..(n - 1)).rev() {
            dat[i] = (multiply)(dat[i * 2 + 1], dat[i * 2 + 2]);
        }
        Self {
            n,
            dat,
            e,
            multiply,
            laz: vec![id; n * 2 - 1],
            id,
            composite,
            apply,
        }
    }
    fn update_node(&mut self, i: usize, f: F) {
        self.dat[i] = (self.apply)(f, self.dat[i]);
        if i < self.n {
            self.laz[i] = (self.composite)(f, self.laz[i]);
        }
    }
    fn update_range(
        &mut self,
        range: &std::ops::Range<usize>,
        i: usize,
        i_range: std::ops::Range<usize>,
        f: F,
    ) {
        if range.end <= i_range.start || i_range.end <= range.start {
            return;
        }
        if range.start <= i_range.start && i_range.end <= range.end {
            self.update_node(i, f);
            return;
        }
        let left_child = i * 2 + 1;
        let right_child = i * 2 + 2;
        self.update_node(left_child, self.laz[i]);
        self.update_node(right_child, self.laz[i]);
        self.laz[i] = self.id;
        let m = (i_range.start + i_range.end) / 2;
        self.update_range(range, left_child, i_range.start..m, f);
        self.update_range(range, right_child, m..i_range.end, f);
        self.dat[i] = (self.multiply)(self.dat[left_child], self.dat[right_child]);
    }
    fn update(&mut self, range: std::ops::Range<usize>, f: F) {
        self.update_range(&range, 0, 0..self.n, f);
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
        let left_result = self._fold(range, i * 2 + 1, i_range.start..m);
        let right_result = self._fold(range, i * 2 + 2, m..i_range.end);
        (self.apply)(self.laz[i], (self.multiply)(left_result, right_result))
    }
    fn fold(&self, range: std::ops::Range<usize>) -> T {
        self._fold(&range, 0, 0..self.n)
    }
    fn get(&self, i: usize) -> T {
        let mut i = i + self.n - 1;
        let mut res = (self.apply)(self.laz[i], self.dat[i]);
        while i > 0 {
            i = (i - 1) / 2;
            res = (self.apply)(self.laz[i], res);
        }
        res
    }
    #[allow(dead_code)]
    fn debug_tree(&self) {
        let mut que = std::collections::VecDeque::new();
        que.push_back(0);
        let mut cnt: usize = 0;
        while let Some(i) = que.pop_front() {
            print!("{:?} ", self.laz[i]);
            cnt += 1;
            if (cnt + 1).is_power_of_two() {
                print!("\n");
            }
            if i * 2 + 2 < self.laz.len() {
                que.push_back(i * 2 + 1);
                que.push_back(i * 2 + 2);
            }
        }
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
