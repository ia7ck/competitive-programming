use input_i_scanner::InputIScanner;

macro_rules! chmin {
    ($a: expr, $b: expr) => {
        $a = std::cmp::min($a, $b);
    };
}

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    macro_rules! scan {
        (($($t: ty),+)) => {
            ($(scan!($t)),+)
        };
        ($t: ty) => {
            _i_i.scan::<$t>() as $t
        };
        (($($t: ty),+); $n: expr) => {
            std::iter::repeat_with(|| scan!(($($t),+))).take($n).collect::<Vec<_>>()
        };
        ($t: ty; $n: expr) => {
            std::iter::repeat_with(|| scan!($t)).take($n).collect::<Vec<_>>()
        };
    }

    let (h, w, k) = scan!((usize, usize, usize));
    let mut a = vec![vec![]; h];
    for i in 0..h {
        a[i] = scan!(u64; w);
    }
    let a = a;

    let mut coordinates = Vec::new();
    for i in 0..h {
        for j in 0..w {
            coordinates.push((i, j));
        }
    }
    coordinates.sort_by_key(|&(i, j)| a[i][j]);
    coordinates.reverse();
    let mut ok = vec![vec![false; w]; h];
    let inf = std::u64::MAX / 2;
    let mut ans = inf;
    for (ii, jj) in coordinates {
        ok[ii][jj] = true;
        let mut dp = vec![vec![vec![inf; k + 1]; w]; h];
        if ok[0][0] {
            dp[0][0][1] = a[0][0];
        } else {
            dp[0][0][0] = 0;
        }
        for i in 0..h {
            for j in 0..w {
                for t in 0..=k {
                    if i >= 1 {
                        if ok[i][j] {
                            if t >= 1 {
                                chmin!(dp[i][j][t], dp[i - 1][j][t - 1] + a[i][j]);
                            }
                        } else {
                            chmin!(dp[i][j][t], dp[i - 1][j][t]);
                        }
                    }
                    if j >= 1 {
                        if ok[i][j] {
                            if t >= 1 {
                                chmin!(dp[i][j][t], dp[i][j - 1][t - 1] + a[i][j]);
                            }
                        } else {
                            chmin!(dp[i][j][t], dp[i][j - 1][t]);
                        }
                    }
                }
            }
        }
        chmin!(ans, dp[h - 1][w - 1][k]);
    }
    println!("{}", ans);
}
