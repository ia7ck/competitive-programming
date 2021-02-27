fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let h: usize = rd.get();
    let w: usize = rd.get();
    let s: Vec<Vec<char>> = (0..h).map(|_| rd.get_chars()).collect();

    const DIR8: [(isize, isize); 8] = [
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut uf = UnionFind::new(h * w);
    for i in 0..h {
        for j in 0..w {
            if s[i][j] != 'o' {
                continue;
            }
            for (ni, nj) in around(i, j).y_range(0..h).x_range(0..w).directions(&DIR8) {
                if s[ni][nj] != 'o' {
                    continue;
                }
                uf.unite(i * w + j, ni * w + nj);
            }
        }
    }
    let mut leaders: Vec<usize> = (0..h * w).map(|v| uf.find(v)).collect();
    leaders.sort();
    leaders.dedup();
    let size: Vec<usize> = leaders
        .into_iter()
        .map(|v| uf.get_size(v))
        .filter(|&sz| sz > 1)
        .collect();
    let reduce = |x| {
        let mut x = x;
        for y in (2..=x).rev() {
            if x % (y * y) == 0 {
                x /= y * y;
            }
        }
        x
    };
    let (mut a, mut b, mut c) = (0, 0, 0);
    // A -> 12, B -> 16, C -> 11
    for sz in size {
        eprintln!("{} {}", sz, reduce(sz));
        match reduce(sz) {
            3 => {
                a += 1;
            }
            1 => {
                b += 1;
            }
            11 => {
                c += 1;
            }
            _ => {}
        }
    }
    println!("{} {} {}", a, b, c);
}

pub struct Around<'a> {
    y: usize,
    x: usize,
    y_range: std::ops::Range<usize>,
    x_range: std::ops::Range<usize>,
    directions: &'a [(isize, isize)],
    dir_idx: usize,
}

pub fn around<'a>(y: usize, x: usize) -> Around<'a> {
    Around {
        y,
        x,
        y_range: 0..std::usize::MAX,
        x_range: 0..std::usize::MAX,
        directions: &[],
        dir_idx: 0,
    }
}

impl<'a> Around<'a> {
    pub fn y_range(self, y_rng: impl std::ops::RangeBounds<usize>) -> Self {
        Self {
            y_range: half_open_range(y_rng),
            ..self
        }
    }
    pub fn x_range(self, x_rng: impl std::ops::RangeBounds<usize>) -> Self {
        Self {
            x_range: half_open_range(x_rng),
            ..self
        }
    }
    pub fn directions(self, dirs: &'a [(isize, isize)]) -> Self {
        Self {
            directions: dirs,
            ..self
        }
    }
}

impl<'a> Iterator for Around<'a> {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        fn dest(u: usize, i: isize) -> Option<usize> {
            if i.is_positive() {
                u.checked_add(i as usize)
            } else {
                u.checked_sub((-i) as usize)
            }
        }
        while let Some(&(dy, dx)) = self.directions.get(self.dir_idx) {
            self.dir_idx += 1;
            if let Some(ny) = dest(self.y, dy) {
                if let Some(nx) = dest(self.x, dx) {
                    if self.y_range.contains(&self.y)
                        && self.x_range.contains(&self.x)
                        && self.y_range.contains(&ny)
                        && self.x_range.contains(&nx)
                    {
                        return Some((ny, nx));
                    }
                }
            }
        }
        None
    }
}

fn half_open_range(rng: impl std::ops::RangeBounds<usize>) -> std::ops::Range<usize> {
    use std::ops::Bound::{Excluded, Included, Unbounded};
    let start = match rng.start_bound() {
        Included(&s) => s,
        Excluded(&s) => s + 1,
        Unbounded => 0,
    };
    let end = match rng.end_bound() {
        Included(&e) => e + 1,
        Excluded(&e) => e,
        Unbounded => std::usize::MAX,
    };
    start..end
}

pub struct UnionFind {
    par: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            par: (0..n).collect(),
            size: vec![1; n],
        }
    }
    pub fn find(&mut self, i: usize) -> usize {
        if self.par[i] == i {
            self.par[i]
        } else {
            self.par[i] = self.find(self.par[i]);
            self.par[i]
        }
    }
    pub fn unite(&mut self, i: usize, j: usize) {
        let i = self.find(i);
        let j = self.find(j);
        if i == j {
            return;
        }
        let (i, j) = if self.size[i] >= self.size[j] {
            (i, j)
        } else {
            (j, i)
        };
        self.par[j] = i;
        self.size[i] += self.size[j];
    }
    pub fn get_size(&mut self, i: usize) -> usize {
        let p = self.find(i);
        self.size[p]
    }
    pub fn same(&mut self, i: usize, j: usize) -> bool {
        self.find(i) == self.find(j)
    }
}

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
