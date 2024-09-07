use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
    };

    let mut a = vec![];
    for i in 0..n {
        input! {
            b: [Usize1; i + 1],
        };
        a.push(b);
    }

    let mut x = 0;
    for y in 0..n {
        if x >= y {
            x = a[x][y];
        } else {
            x = a[y][x];
        }
    }

    println!("{}", x + 1);
}
