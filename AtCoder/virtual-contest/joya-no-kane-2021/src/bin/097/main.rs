use std::collections::VecDeque;

use grid_search::around;
use input_i_scanner::InputIScanner;

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    macro_rules! scan {
        (($($t: ty),+)) => {
            ($(scan!($t)),+)
        };
        ($t: ty) => {
            _i_i.scan::<$t>() as $t
        };
        (($($t: ty),+); $n: expr) => {
            std::iter::repeat_with(|| scan!(($($t),+))).take($n).collect::<Vec<_>>()
        };
        ($t: ty; $n: expr) => {
            std::iter::repeat_with(|| scan!($t)).take($n).collect::<Vec<_>>()
        };
    }

    let (h, w) = scan!((usize, usize));
    let a: Vec<Vec<char>> = std::iter::repeat_with(|| {
        let row = scan!(String);
        row.chars().collect()
    })
    .take(h)
    .collect();

    let dirs = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    let mut d = vec![vec![std::u32::MAX; w]; h];
    d[0][0] = 0;
    let mut que = VecDeque::new();
    que.push_back((0, 0));
    while let Some((i, j)) = que.pop_front() {
        for (ni, nj) in around(i, j).y_range(0..h).x_range(0..w).directions(&dirs) {
            if a[ni][nj] == '.' && d[ni][nj] == std::u32::MAX {
                d[ni][nj] = d[i][j] + 1;
                que.push_back((ni, nj));
            }
        }
    }
    let d = d[h - 1][w - 1];
    if d == std::u32::MAX {
        println!("-1");
        return;
    }
    let mut walls = 0;
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '#' {
                walls += 1;
            }
        }
    }
    let ans = h * w - walls - (d + 1) as usize;
    println!("{}", ans);
}
