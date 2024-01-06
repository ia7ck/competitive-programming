use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut a = vec![vec![0; n]; n];
    let n = n as isize;
    let (mut i, mut j) = (0, 0);
    let (mut di, mut dj) = (0, 1);
    let mut x = 1;
    while a[i as usize][j as usize] == 0 {
        a[i as usize][j as usize] = x;
        x += 1;
        (di, dj) = if 0 <= i + di
            && i + di < n
            && 0 <= j + dj
            && j + dj < n
            && a[(i + di) as usize][(j + dj) as usize] == 0
        {
            (di, dj)
        } else {
            (dj, -di)
        };
        (i, j) = (i + di, j + dj);
    }

    for i in 0..n {
        for j in 0..n {
            let x = a[i as usize][j as usize];
            if x == n * n {
                print!("T");
            } else {
                print!("{}", x);
            }
            if j < n - 1 {
                print!(" ");
            } else {
                print!("\n");
            }
        }
    }
}
