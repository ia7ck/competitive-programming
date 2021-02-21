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

#[derive(Debug, Clone)]
struct Edge {
    to: usize,
    cost: i64,
}

fn bfs(g: &Vec<Vec<Edge>>, s: usize, inf: i64) -> (Vec<i64>, Vec<Option<usize>>) {
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

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let h: usize = rd.get();
    let w: usize = rd.get();
    let sy: usize = rd.get();
    let sx: usize = rd.get();
    let gy: usize = rd.get();
    let gx: usize = rd.get();
    let sy = sy - 1;
    let sx = sx - 1;
    let gy = gy - 1;
    let gx = gx - 1;
    let a: Vec<Vec<char>> = (0..h)
        .map(|_| {
            let s: String = rd.get();
            s.chars().collect()
        })
        .collect();
    let inf = std::i64::MAX / 2;
    let mut g = vec![vec![]; h * w];
    let id = |(i, j): (usize, usize)| -> usize { i * w + j };
    let dist = |(x1, y1): (usize, usize), (x2, y2): (usize, usize)| {
        (x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs()
    };
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '#' {
                continue;
            }
            for ni in (i as i32 - 2)..=(i as i32 + 2) {
                for nj in (j as i32 - 2)..=(j as i32 + 2) {
                    if ni < 0 || ni >= h as i32 || nj < 0 || nj >= w as i32 {
                        continue;
                    }
                    let ni = ni as usize;
                    let nj = nj as usize;
                    if i == ni && j == nj {
                        continue;
                    }
                    if a[ni][nj] == '#' {
                        continue;
                    }
                    let u = id((i, j));
                    let v = id((ni, nj));
                    if dist((i, j), (ni, nj)) == 1 {
                        g[u].push(Edge { to: v, cost: 0 });
                        g[v].push(Edge { to: u, cost: 0 });
                    } else {
                        g[u].push(Edge { to: v, cost: 1 });
                        g[v].push(Edge { to: u, cost: 1 });
                    }
                }
            }
        }
    }
    let (d, _) = bfs(&g, id((sy, sx)), inf);
    let mut ans = d[id((gy, gx))];
    if ans == inf {
        ans = -1;
    }
    println!("{}", ans);
}
