use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        positions: [(Usize1, Usize1); n - 1],
    };

    let mut bad = vec![vec![false; n]; n];
    for (r, c) in positions {
        bad[r][c] = true;
        for i in 0..n {
            bad[i][c] = true;
        }
        for j in 0..n {
            bad[r][j] = true;
        }
        // ななめ方向を bad[*][*] = true にする
        for (i, j) in (0..r).rev().zip((0..c).rev()) {
            bad[i][j] = true;
        }
        for (i, j) in ((r + 1)..n).zip(c + 1..n) {
            bad[i][j] = true;
        }
        for (i, j) in (0..r).rev().zip((c + 1)..n) {
            bad[i][j] = true;
        }
        for (i, j) in ((r + 1)..n).zip((0..c).rev()) {
            bad[i][j] = true;
        }
    }
    for i in 0..n {
        for j in 0..n {
            if bad[i][j] == false {
                println!("{} {}", i + 1, j + 1);
                return;
            }
        }
    }
    println!("-1");
}
