use proconio::{input, marker::Chars};
use grid_search::around;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    };

    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '.' {
                let mut cookie = 0;
                for (ni, nj) in around(i, j).y_range(0..h).x_range(0..w).directions(&[(-1, 0), (0, -1), (1, 0), (0, 1)]) {
                    if a[ni][nj] == '#' {
                        cookie += 1;
                    }
                }
                if cookie >= 2 {
                    println!("{} {}", i + 1, j + 1);
                    return;
                }
            }
        }
    }
    unreachable!();
}
