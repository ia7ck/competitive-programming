use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    };

    let f = |i: usize, x: usize, y: usize| -> (usize, usize) {
        match i % 4 {
            0 => (x, y),
            1 => (y, n - x - 1),
            2 => (n - x - 1, n - y - 1),
            3 => (n - y - 1, x),
            _ => unreachable!(),
        }
    };

    let mut b = a.clone();
    for i in 0..(n / 2) {
        for x in [i, n - i - 1] {
            for y in i..(n - i) {
                let (nx, ny) = f(i + 1, x, y);
                b[nx][ny] = a[x][y];
            }
        }
        for x in i..(n - i) {
            for y in [i, n - i - 1] {
                let (nx, ny) = f(i + 1, x, y);
                b[nx][ny] = a[x][y];
            }
        }
    }

    for row in b {
        println!("{}", row.iter().collect::<String>());
    }
}
