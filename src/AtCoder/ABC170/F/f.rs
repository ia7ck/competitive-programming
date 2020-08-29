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

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let h: usize = rd.get();
    let w: usize = rd.get();
    let k: i64 = rd.get();
    let sy: usize = rd.get();
    let sx: usize = rd.get();
    let gy: usize = rd.get();
    let gx: usize = rd.get();
    let a: Vec<Vec<char>> = (0..h)
        .map(|_| rd.get::<String>().chars().collect())
        .collect();

    let sy = sy - 1;
    let sx = sx - 1;
    let gy = gy - 1;
    let gx = gx - 1;

    let dy = vec![-1, 0, 0, 1];
    let dx = vec![0, -1, 1, 0];

    let inf = std::i64::MAX / 3;
    let mut d = vec![vec![vec![inf; 4]; w]; h];
    let mut q = std::collections::BinaryHeap::new();
    use std::cmp::Reverse;
    for p in 0..4 {
        d[sy][sx][p] = 0;
        q.push((Reverse(0), (sy, sx, p)));
    }
    while let Some((Reverse(c), (y, x, p))) = q.pop() {
        if c > d[y][x][p] {
            continue;
        }
        for np in 0..4 {
            let nd = (c + k - 1) / k * k;
            if nd < d[y][x][np] {
                d[y][x][np] = nd;
                q.push((Reverse(nd), (y, x, np)));
            }
        }
        {
            // 同じ方向に進む
            let ny = y as i32 + dy[p];
            let nx = x as i32 + dx[p];
            if nx < 0 || ny < 0 {
                continue;
            }
            let ny = ny as usize;
            let nx = nx as usize;
            if ny >= h || nx >= w || a[ny][nx] == '@' {
                continue;
            }
            if c + 1 < d[ny][nx][p] {
                d[ny][nx][p] = c + 1;
                q.push((Reverse(d[ny][nx][p]), (ny, nx, p)));
            }
        }
    }
    let mut ans = inf;
    for p in 0..4 {
        if d[gy][gx][p] < inf {
            ans = std::cmp::min(ans, (d[gy][gx][p] + k - 1) / k);
        }
    }
    if ans == inf {
        ans = -1;
    }
    println!("{}", ans);
}
