fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let h: usize = rd.get();
    let w: usize = rd.get();
    let n: usize = rd.get();
    let m: usize = rd.get();
    let lights: Vec<(usize, usize)> = (0..n)
        .map(|_| {
            let a: usize = rd.get();
            let b: usize = rd.get();
            (a - 1, b - 1)
        })
        .collect();
    let blocks: Vec<(usize, usize)> = (0..m)
        .map(|_| {
            let a: usize = rd.get();
            let b: usize = rd.get();
            (a - 1, b - 1)
        })
        .collect();

    let mut seen_ud = vec![vec![false; w]; h];
    let mut seen_lr = vec![vec![false; w]; h];
    for &(i, j) in &lights {
        seen_ud[i][j] = true;
        seen_lr[i][j] = true;
    }
    let mut ban = vec![vec![false; w]; h];
    for &(i, j) in &blocks {
        ban[i][j] = true;
    }
    const UD: [(isize, isize); 2] = [(1, 0), (-1, 0)];
    const LR: [(isize, isize); 2] = [(0, -1), (0, 1)];
    // const UDLR: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, -1), (0, 1)];
    for &(i, j) in &lights {
        let mut q = std::collections::VecDeque::new();
        q.push_back((i, j));
        while let Some((cur_i, cur_j)) = q.pop_front() {
            for (ni, nj) in Adjacent::new((cur_i, cur_j), h, w, UD.iter().copied()) {
                if ban[ni][nj] {
                    continue;
                }
                if !seen_ud[ni][nj] {
                    seen_ud[ni][nj] = true;
                    q.push_back((ni, nj));
                }
            }
        }
        let mut q = std::collections::VecDeque::new();
        q.push_back((i, j));
        while let Some((cur_i, cur_j)) = q.pop_front() {
            for (ni, nj) in Adjacent::new((cur_i, cur_j), h, w, LR.iter().copied()) {
                if ban[ni][nj] {
                    continue;
                }
                if !seen_lr[ni][nj] {
                    seen_lr[ni][nj] = true;
                    q.push_back((ni, nj));
                }
            }
        }
    }
    let mut ans = 0;
    for i in 0..h {
        // println!("{:?}", seen[i]);
        for j in 0..w {
            if seen_ud[i][j] || seen_lr[i][j] {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

/// グリッドグラフで現在位置の周辺を走査したいときに使えます。
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
    /// 隣接 4 方向を走査する例です。
    /// # Examples
    /// ```
    /// use grid::Adjacent;
    /// const NSEW: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];
    /// let adjs = Adjacent::new((0, 1), 3, 4, NSEW.iter().copied()).collect::<Vec<_>>();
    /// assert_eq!(vec![(1, 1), (0, 2), (0, 0)], adjs);
    /// // .x..
    /// // ....
    /// // ....
    /// //
    /// //  |
    /// //  v
    /// //
    /// // x.x.
    /// // .x..
    /// // ....
    /// ```
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
