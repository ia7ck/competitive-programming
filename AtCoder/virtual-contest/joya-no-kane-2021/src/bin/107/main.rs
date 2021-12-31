use std::collections::VecDeque;

use input_i_scanner::InputIScanner;
use join::Join;

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
    let n = scan!(usize);
    let a = scan!(u32; n);
    let mut que = VecDeque::new();
    for i in 0..n {
        for _ in 0..a[i] {
            que.push_back(i + 1);
        }
    }
    let mut y = 0;
    let mut x = 0;
    let mut right = true;
    let mut ans = vec![vec![0; w]; h];
    while let Some(color) = que.pop_front() {
        ans[y][x] = color;
        if right {
            if x + 1 < w {
                x += 1;
            } else {
                y += 1;
                right = false;
            }
        } else {
            if x >= 1 {
                x -= 1;
            } else {
                y += 1;
                right = true;
            }
        }
    }
    for row in ans {
        println!("{}", row.iter().join(" "));
    }
}
