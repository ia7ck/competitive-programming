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

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let inf = std::usize::MAX;
    let e = inf;
    let mut a = vec![e; n + 1];
    for i in 1..=(n - 1) {
        a[i] = n - 2;
    }
    let mult = |a, b| std::cmp::min(a, b);
    let id = inf - 1;
    let com = |f, g| {
        if f == id {
            g
        } else {
            std::cmp::min(f, g)
        }
    };
    let apply = |f, a| {
        if f == id {
            a
        } else {
            std::cmp::min(f, a)
        }
    };
    let mut seg1 = LazySegmentTree::new(&a, e, mult, id, com, apply);
    let mut seg2 = LazySegmentTree::new(&a, e, mult, id, com, apply);
    let q: usize = rd.get();
    let mut ans = (n - 2) * (n - 2);
    for _ in 0..q {
        let t: usize = rd.get();
        let x: usize = rd.get();
        match t {
            1 => {
                let k = seg1.get(x);
                seg1.update(x..(x + 1), 0);
                ans -= k;
                seg2.update(2..(k + 2), x - 2);
            }
            2 => {
                let k = seg2.get(x);
                seg2.update(x..(x + 1), 0);
                ans -= k;
                seg1.update(2..(k + 2), x - 2);
            }
            _ => {
                unreachable!();
            }
        }
    }
    println!("{}", ans);
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
