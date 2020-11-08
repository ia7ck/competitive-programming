extern crate proconio;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        (R, C): (usize, usize),
        (sy, sx): (usize, usize),
        (gy, gx): (usize, usize),
        a: [Chars; R],
    }
    let (sy, sx) = (sy - 1, sx - 1);
    let (gy, gx) = (gy - 1, gx - 1);
    let mut d = vec![vec![-1; C]; R];
    let dydx = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let mut q = std::collections::VecDeque::new();
    d[sy][sx] = 0;
    q.push_back((sy, sx));
    while let Some((y, x)) = q.pop_front() {
        for (ny, nx) in Adjacent::new((y, x), R, C, dydx.iter().copied()) {
            if a[ny][nx] == '.' && d[ny][nx] == -1 {
                d[ny][nx] = d[y][x] + 1;
                q.push_back((ny, nx));
            }
        }
    }
    println!("{}", d[gy][gx]);
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
