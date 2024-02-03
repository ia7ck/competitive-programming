use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
    };

    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut a = vec![vec!['.'; w]; h];
    let (mut i, mut j, mut d) = (0, 0, 0);
    for _ in 0..n {
        if a[i][j] == '.' {
            a[i][j] = '#';
            d = (d + 1) % 4;
        } else {
            a[i][j] = '.';
            d = (4 + d - 1) % 4;
        }
        let (di, dj) = dirs[d];
        (i, j) = (
            (h as isize + i as isize + di) as usize % h,
            (w as isize + j as isize + dj) as usize % w,
        );
    }

    for row in a {
        println!("{}", row.into_iter().collect::<String>());
    }
}
