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
    let inf = std::u64::MAX / 2;
    let mut d = vec![vec![inf; w]; h];
    let mut q = std::collections::VecDeque::new();
    d[sy][sx] = 0;
    q.push_back((sy, sx));

    const D1: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    #[rustfmt::skip]
    const D2: [(isize, isize); 24] = [
        (-2, -2), (-2, -1), (-2, 0), (-2, 1), (-2, 2),
        (-1, -2), (-1, -1), (-1, 0), (-1, 1), (-1, 2),
        ( 0, -2), ( 0, -1),          ( 0, 1), ( 0, 2),
        ( 1, -2), ( 1, -1), ( 1, 0), ( 1, 1), ( 1, 2),
        ( 2, -2), ( 2, -1), ( 2, 0), ( 2, 1), ( 2, 2),
    ];

    while let Some((y, x)) = q.pop_front() {
        for (ny, nx) in Adjacent::new((y, x), h, w, D1.iter().copied()) {
            if a[ny][nx] == '.' {
                if d[ny][nx] > d[y][x] {
                    d[ny][nx] = d[y][x];
                    q.push_front((ny, nx));
                }
            }
        }
        for (ny, nx) in Adjacent::new((y, x), h, w, D2.iter().copied()) {
            if a[ny][nx] == '.' {
                if d[ny][nx] > d[y][x] + 1 {
                    d[ny][nx] = d[y][x] + 1;
                    q.push_back((ny, nx));
                }
            }
        }
    }
    let ans = d[gy][gx];
    if ans < inf {
        println!("{}", ans);
    } else {
        println!("{}", -1);
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
