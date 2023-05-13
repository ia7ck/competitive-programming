use proconio::{input, marker::Chars};

fn f(a: &Vec<Vec<char>>, s: usize, t: usize) -> Vec<Vec<char>> {
    let mut b = a.clone();
    let h = b.len();
    let w = b[0].len();
    for _ in 0..s {
        for j in 0..w {
            let x = b[0][j];
            for i in 0..(h - 1) {
                b[i][j] = b[i + 1][j];
            }
            b[h - 1][j] = x;
        }
    }
    for _ in 0..t {
        for i in 0..h {
            let x = b[i][0];
            for j in 0..(w - 1) {
                b[i][j] = b[i][j + 1];
            }
            b[i][w - 1] = x;
        }
    }
    b
}

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
        b: [Chars; h],
    };

    for s in 0..h {
        for t in 0..w {
            let c = f(&a, s, t);
            if b == c {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
