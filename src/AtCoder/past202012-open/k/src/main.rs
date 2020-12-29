use std::collections::HashMap;

const DIR: [(isize, isize); 5] = [(-1, 0), (0, 1), (1, 0), (0, -1), (0, 0)];

fn solve(s: &mut Vec<Vec<char>>, memo: &mut HashMap<Vec<Vec<char>>, f64>) -> f64 {
    let mut all = true;
    for i in 0..4 {
        for j in 0..4 {
            if s[i][j] == '#' {
                all = false;
            }
        }
    }
    if all {
        return 0.0;
    }
    if let Some(&ans) = memo.get(s) {
        return ans;
    }
    let mut ans: f64 = 1e9;
    for i in 0..4 {
        for j in 0..4 {
            let mato: Vec<(usize, usize)> = Adjacent::new((i, j), 4, 4, DIR.iter().copied())
                .filter(|&(y, x)| s[y][x] == '#')
                .collect();
            if mato.is_empty() {
                continue;
            }
            let mut tmp = (5 - mato.len()) as f64 / 5.0;
            for &(y, x) in &mato {
                s[y][x] = '.';
                tmp += (1.0 + solve(s, memo)) / 5.0;
                s[y][x] = '#';
            }
            ans = ans.min(tmp * 5.0 / mato.len() as f64);
        }
    }
    memo.insert(s.clone(), ans);
    ans
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = 4;
    let mut s: Vec<Vec<char>> = (0..n)
        .map(|_| rd.get::<String>().chars().collect())
        .collect();
    let mut memo = HashMap::new();
    let ans = solve(&mut s, &mut memo);
    println!("{:.10}", ans);
}

pub struct Adjacent<I> {
    position: (usize, usize),
    h: usize,
    w: usize,
    direction: I,
}

impl<I> Adjacent<I>
where
    I: Iterator<Item = (isize, isize)>,
{
    pub fn new(position: (usize, usize), h: usize, w: usize, direction: I) -> Self {
        Self {
            position,
            h,
            w,
            direction,
        }
    }
}

impl<I> Iterator for Adjacent<I>
where
    I: Iterator<Item = (isize, isize)>,
{
    type Item = (usize, usize);
    fn next(&mut self) -> Option<(usize, usize)> {
        while let Some((di, dj)) = self.direction.next() {
            let (i, j) = self.position;
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
