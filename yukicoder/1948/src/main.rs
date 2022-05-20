use proconio::input;

macro_rules! chmax {
    ($a: expr, $b: expr) => {
        $a = $a.max($b);
    };
}

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[u64; w]; h],
    };

    let mut dp = vec![vec![vec![0; 2]; w]; h];
    dp[0][0][0] = a[0][0];
    for i in 0..h {
        for j in 0..w {
            for (ni, nj) in [(i + 1, j), (i, j + 1)] {
                if ni < h && nj < w {
                    if dp[i][j][0] > a[ni][nj] {
                        chmax!(dp[ni][nj][0], dp[i][j][0] + a[ni][nj]);
                    } else {
                        if ni != h - 1 || nj != w - 1 {
                            chmax!(dp[ni][nj][1], dp[i][j][0]);
                        }
                    }
                    if dp[i][j][1] > a[ni][nj] {
                        chmax!(dp[ni][nj][1], dp[i][j][1] + a[ni][nj]);
                    }
                }
            }
        }
    }
    if dp[h-1][w-1][0] != 0 || dp[h-1][w-1][1] != 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
