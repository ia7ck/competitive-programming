use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        h: usize,
        w: usize,
        si: Usize1,
        sj: Usize1,
        c: [Chars; h],
        x: Chars,
    };

    let (mut i, mut j) = (si, sj);
    for x in x {
        match x {
            'U' => {
                if i > 0 && c[i - 1][j] != '#' {
                    i -= 1;
                }
            }
            'D' => {
                if i < h - 1 && c[i + 1][j] != '#' {
                    i += 1;
                }
            }
            'L' => {
                if j > 0 && c[i][j - 1] != '#' {
                    j -= 1;
                }
            }
            'R' => {
                if j < w - 1 && c[i][j + 1] != '#' {
                    j += 1;
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{} {}", i + 1, j + 1);
}
