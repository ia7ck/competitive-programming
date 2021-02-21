fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let h: usize = rd.get();
    let w: usize = rd.get();
    let sx: usize = rd.get();
    let sy: usize = rd.get();
    let gx: usize = rd.get();
    let gy: usize = rd.get();
    let a: Vec<Vec<char>> = (0..h)
        .map(|_| rd.get::<String>().chars().collect())
        .collect();

    const NSEW: [(isize, isize); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];
    let inf = std::u64::MAX;
    let mut d = vec![vec![vec![inf; 2]; w]; h];
    let sx = sx - 1;
    let sy = sy - 1;
    d[sx][sy][0] = 0;
    d[sx][sy][1] = 0;
    use std::collections::VecDeque;
    let mut q = VecDeque::new();
    q.push_back((sx, sy, 0));
    q.push_back((sx, sy, 1));
    while let Some((i, j, mv)) = q.pop_front() {
        for (ni, nj) in Adjacent::new((i, j), h, w, &NSEW) {
            if a[ni][nj] == '#' {
                continue;
            }
            match mv {
                0 => {
                    if i == ni && d[ni][nj][1] == inf {
                        d[ni][nj][1] = d[i][j][0] + 1;
                        q.push_back((ni, nj, 1));
                    }
                }
                1 => {
                    if j == nj && d[ni][nj][0] == inf {
                        d[ni][nj][0] = d[i][j][1] + 1;
                        q.push_back((ni, nj, 0));
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    let gx = gx - 1;
    let gy = gy - 1;
    let ans = std::cmp::min(d[gx][gy][0], d[gx][gy][1]);
    if ans == std::u64::MAX {
        println!("{}", -1);
    } else {
        println!("{}", ans);
    }
}

pub struct Adjacent<'a> {
    position: (usize, usize),
    h: usize,
    w: usize,
    direction: &'a [(isize, isize)],
    idx: usize,
}

impl<'a> Adjacent<'a> {
    pub fn new(
        position: (usize, usize),
        h: usize,
        w: usize,
        direction: &'a [(isize, isize)],
    ) -> Self {
        Self {
            position,
            h,
            w,
            direction,
            idx: 0,
        }
    }
}

impl<'a> Iterator for Adjacent<'a> {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<(usize, usize)> {
        let (i, j) = self.position;
        while let Some((di, dj)) = self.direction.get(self.idx) {
            self.idx += 1;
            let ni = i as isize + di;
            let nj = j as isize + dj;
            if 0 <= ni && ni < self.h as isize && 0 <= nj && nj < self.w as isize {
                return Some((ni as usize, nj as usize));
            }
        }
        None
    }
}

pub struct ProconReader<R> {
    r: R,
    line: String,
    i: usize,
}

impl<R: std::io::BufRead> ProconReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            r: reader,
            line: String::new(),
            i: 0,
        }
    }
    pub fn get<T>(&mut self) -> T
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.skip_blanks();
        assert!(self.i < self.line.len());
        assert_ne!(&self.line[self.i..=self.i], " ");
        let line = &self.line[self.i..];
        let end = line.find(' ').unwrap_or_else(|| line.len());
        let s = &line[..end];
        self.i += end;
        s.parse()
            .unwrap_or_else(|_| panic!("parse error `{}`", self.line))
    }
    fn skip_blanks(&mut self) {
        loop {
            let start = self.line[self.i..].find(|ch| ch != ' ');
            match start {
                Some(j) => {
                    self.i += j;
                    break;
                }
                None => {
                    self.line.clear();
                    self.i = 0;
                    let num_bytes = self.r.read_line(&mut self.line).unwrap();
                    assert!(num_bytes > 0, "reached EOF :(");
                    self.line = self.line.trim_end_matches(&['\r', '\n'][..]).to_string();
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
}
