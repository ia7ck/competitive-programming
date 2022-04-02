use scanner_proc_macro::insert_scanner;
use std::collections::VecDeque;

#[insert_scanner]
fn main() {
    let n = scan!(usize);

    let (sy, sx) = scan!((usize, usize));
    let (gy, gx) = scan!((usize, usize));
    let a: Vec<Vec<char>> = (0..n)
        .map(|_| {
            let s = scan!(String);
            s.chars().collect()
        })
        .collect();

    let dirs = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

    let inf = std::u64::MAX;
    let mut d = vec![vec![vec![inf; 5]; n]; n];
    let mut que = VecDeque::new();
    d[sy - 1][sx - 1][4] = 0;
    que.push_back(((sy - 1, sx - 1), 4));
    while let Some(((y, x), dir)) = que.pop_front() {
        for (i, &(dy, dx)) in dirs.iter().enumerate() {
            let ny = y as isize + dy;
            let nx = x as isize + dx;
            if ny < 0 || ny >= n as isize || nx < 0 || nx >= n as isize {
                continue;
            }
            let ny = ny as usize;
            let nx = nx as usize;
            if a[ny][nx] == '#' {
                continue;
            }
            if dir == i {
                if d[ny][nx][dir] > d[y][x][dir] {
                    d[ny][nx][dir] = d[y][x][dir];
                    que.push_front(((ny, nx), dir));
                }
            } else {
                if d[ny][nx][i] > d[y][x][dir] + 1 {
                    d[ny][nx][i] = d[y][x][dir] + 1;
                    que.push_back(((ny, nx), i));
                }
            }
        }
    }

    let mut ans = inf;
    for dir in 0..4 {
        ans = ans.min(d[gy - 1][gx - 1][dir]);
    }
    if ans == inf {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
