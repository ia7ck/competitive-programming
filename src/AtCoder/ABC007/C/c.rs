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
        for (dy, dx) in &dydx {
            let ny = y as isize + dy;
            let nx = x as isize + dx;
            if 0 <= ny && ny < R as isize && 0 <= nx && nx < C as isize {
                let ny = ny as usize;
                let nx = nx as usize;
                if a[ny][nx] == '.' && d[ny][nx] == -1 {
                    d[ny][nx] = d[y][x] + 1;
                    q.push_back((ny, nx));
                }
            }
        }
    }
    println!("{}", d[gy][gx]);
}
