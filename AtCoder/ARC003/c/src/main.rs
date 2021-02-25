fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let _: usize = rd.get();
    let a: Vec<Vec<char>> = (0..n).map(|_| rd.get_chars()).collect();

    if !reachable(0.0, &a) {
        println!("-1");
        return;
    }

    let mut ng = 10.0;
    let mut ok = 0.0;
    for _ in 0..100 {
        let lb = (ng + ok) / 2.0;
        if reachable(lb, &a) {
            ok = lb;
        } else {
            ng = lb;
        }
    }
    println!("{:10}", ok);
}

fn reachable(lb: f64, a: &[Vec<char>]) -> bool {
    let (n, m) = (a.len(), a[0].len());
    let (mut si, mut sj) = (0, 0);
    let (mut gi, mut gj) = (0, 0);
    for i in 0..n {
        for j in 0..m {
            if a[i][j] == 's' {
                si = i;
                sj = j;
            }
            if a[i][j] == 'g' {
                gi = i;
                gj = j;
            }
        }
    }
    const NSEW: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];
    let inf = std::u64::MAX;
    let mut d = vec![vec![inf; m]; n];
    d[si][sj] = 0;
    use std::collections::VecDeque;
    let mut q = VecDeque::new();
    q.push_back((si, sj));
    'outer: while let Some((i, j)) = q.pop_front() {
        for (ni, nj) in around(i, j).y_range(0..n).x_range(0..m).directions(&NSEW) {
            if a[ni][nj] == 'g' {
                d[ni][nj] = d[i][j] + 1;
                break 'outer;
            }
            if a[ni][nj] == '#' {
                continue;
            }
            if d[ni][nj] != inf {
                continue;
            }
            let sun = a[ni][nj] as u64 - '0' as u64;
            let light = (sun as f64) * 0.99_f64.powi((d[i][j] + 1) as i32);
            if light >= lb {
                d[ni][nj] = d[i][j] + 1;
                q.push_back((ni, nj));
            }
        }
    }
    d[gi][gj] < inf
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
