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
    T: Copy + std::fmt::Debug,
    F: Fn(T, T) -> T,
    U: Copy + std::fmt::Debug,
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
        &self,
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

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let h: usize = rd.get();
    let w: usize = rd.get();
    let ab: Vec<(usize, usize)> = (0..h)
        .map(|_| {
            let a: usize = rd.get();
            let b: usize = rd.get();
            (a - 1, b - 1)
        })
        .collect();
    let e_u = std::usize::MAX;
    let mut seg = LazySegmentTree::new(
        &(0..w).zip(0..w).collect(),
        (0, !0),
        |a, b| {
            if a.1 - a.0 < b.1 - b.0 {
                a
            } else {
                b
            }
        },
        e_u,
        |u, v| if u == e_u { v } else { u },
        |u, a| {
            if u == e_u {
                a
            } else {
                (a.0, u)
            }
        },
    );
    for (i, &(a, b)) in ab.iter().enumerate() {
        let lower_bound = |x| {
            if seg.get(0).1 >= x {
                return 0;
            }
            let mut ok = w;
            let mut ng = 0;
            while ok - ng > 1 {
                let m = (ok + ng) / 2;
                if seg.get(m).1 >= x {
                    ok = m;
                } else {
                    ng = m;
                }
            }
            ok
        };
        let l = lower_bound(a);
        let r = lower_bound(b + 1);
        if l < r {
            seg.update(l..r, if b + 1 < w { b + 1 } else { w * 10 });
        }
        // println!("{:?}", (0..w).map(|i| seg.get(i)).collect::<Vec<_>>());
        // seg.debug_tree();
        let (begin, end) = seg.fold(0..w);
        let ans = end - begin;
        if ans < w {
            println!("{}", (i + 1) + ans);
        } else {
            println!("{}", -1);
        }
    }
}
