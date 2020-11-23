fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let h: usize = rd.get();
    let w: usize = rd.get();
    let a: Vec<Vec<char>> = (0..h)
        .map(|_| {
            let s: String = rd.get();
            s.chars().collect()
        })
        .collect();

    let dir = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let idx = |i, j| i * w + j;
    let n = h * w + 26;
    let mut g = vec![vec![]; n];
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '#' {
                continue;
            }
            for (ni, nj) in Adjacent::new((i, j), h, w, dir.iter().cloned()) {
                if a[ni][nj] == '#' {
                    continue;
                }
                g[idx(i, j)].push(Edge {
                    to: idx(ni, nj),
                    cost: 1,
                });
                g[idx(ni, nj)].push(Edge {
                    to: idx(i, j),
                    cost: 1,
                });
            }
            if a[i][j].is_ascii_lowercase() {
                let v = h * w + (a[i][j] as usize - 'a' as usize);
                g[idx(i, j)].push(Edge { to: v, cost: 1 });
                g[v].push(Edge {
                    to: idx(i, j),
                    cost: 0,
                });
            }
        }
    }
    let mut start = 0;
    let mut goal = 0;
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 'S' {
                start = idx(i, j);
            }
            if a[i][j] == 'G' {
                goal = idx(i, j);
            }
        }
    }
    let inf = std::i64::MAX / 2;
    let (dist, _) = bfs_01(&g, start, inf);
    let ans = dist[goal];
    if ans == inf {
        println!("{}", -1);
    } else {
        println!("{}", ans);
    }
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

#[derive(Debug, Clone)]
struct Edge {
    to: usize,
    cost: i64,
}

fn bfs_01(g: &Vec<Vec<Edge>>, s: usize, inf: i64) -> (Vec<i64>, Vec<Option<usize>>) {
    let n = g.len();
    let mut d = vec![inf; n];
    let mut prev = vec![None; n];
    let mut q = std::collections::VecDeque::new();
    d[s] = 0;
    q.push_back((d[s], s));
    while let Some((c, v)) = q.pop_front() {
        if c > d[v] {
            continue;
        }
        for e in &g[v] {
            if c + e.cost < d[e.to] {
                d[e.to] = c + e.cost;
                prev[e.to] = Some(v);
                match e.cost {
                    0 => q.push_front((d[e.to], e.to)),
                    1 => q.push_back((d[e.to], e.to)),
                    _ => unreachable!(),
                }
            }
        }
    }
    (d, prev)
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
