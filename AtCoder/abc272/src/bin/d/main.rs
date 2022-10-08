use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    let mut nexts = Vec::new();
    for dy in -400..=400 {
        for dx in -400..=400 {
            if (dy * dy + dx * dx) as usize == m {
                nexts.push((dy, dx));
            }
        }
    }

    let mut a = vec![vec![-1; n]; n];
    let mut que = VecDeque::new();
    a[0][0] = 0;
    que.push_back((0, 0));
    while let Some((i, j)) = que.pop_front() {
        for &(dy, dx) in &nexts {
            let ni = i as isize + dy;
            let nj = j as isize + dx;
            if 0 <= ni && ni < n as isize && 0 <= nj && nj < n as isize {
                let ni = ni as usize;
                let nj = nj as usize;
                if a[ni][nj] == -1 {
                    a[ni][nj] = a[i][j] + 1;
                    que.push_back((ni, nj));
                }
            }
        }
    }

    for row in a {
        for j in 0..n {
            print!("{}", row[j]);
            if j + 1 < n {
                print!(" ");
            }
        }
        println!();
    }
}
