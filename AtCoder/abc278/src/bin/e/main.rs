use join::Join;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        hh: usize,
        ww: usize,
        a: [[usize; w]; h],
    };

    let mut f = vec![vec![vec![0_i32; n + 1]; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {
            for x in 1..=n {
                f[i + 1][j + 1][x] = f[i][j + 1][x] + f[i + 1][j][x] - f[i][j][x];
            }
            f[i + 1][j + 1][a[i][j]] += 1;
        }
    }
    let mut ans = vec![vec![0; w - ww + 1]; h - hh + 1];
    let mut g = vec![0; n + 1]; // for debug
    for k in 0..=(h - hh) {
        for l in 0..=(w - ww) {
            let mut uniq = 0;
            for x in 1..=n {
                let sub = f[k + hh][l + ww][x] - f[k][l + ww][x] - f[k + hh][l][x] + f[k][l][x];
                g[x] = f[h][w][x] - sub;
                assert!(g[x] >= 0);
                if g[x] >= 1 {
                    uniq += 1;
                }
            }
            ans[k][l] = uniq;
        }
    }

    for row in ans {
        println!("{}", row.iter().join(" "));
    }
}
