use input_i_scanner::InputIScanner;

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

    macro_rules! chmax {
        ($a: expr, $b: expr) => {
            $a = std::cmp::max($a, $b);
        };
    }

    let (h, w) = scan!((usize, usize));
    let a: Vec<Vec<char>> = (0..h)
        .map(|_| {
            let s = scan!(String);
            s.chars().collect()
        })
        .collect();
    let mut dp = vec![vec![0; w]; h];
    dp[0][0] = 1;
    for i in 0..h {
        for j in 0..w {
            if i + 1 < h && a[i + 1][j] == '.' && dp[i][j] >= 1 {
                chmax!(dp[i + 1][j], dp[i][j] + 1);
            }
            if j + 1 < w && a[i][j + 1] == '.' && dp[i][j] >= 1 {
                chmax!(dp[i][j + 1], dp[i][j] + 1);
            }
        }
    }
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            chmax!(ans, dp[i][j]);
        }
    }
    println!("{}", ans);
}
