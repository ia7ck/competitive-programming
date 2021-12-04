use input_i_scanner::InputIScanner;
use join::Join;

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

    let (n, a, b) = scan!((i64, i64, i64));
    let (p, q, r, s) = scan!((i64, i64, i64, i64));

    let mut ans = vec![vec!['.'; (s - r + 1) as usize]; (q - p + 1) as usize];
    for i in p..=q {
        for j in r..=s {
            let k = i - a;
            if (k == j - b && (1 - a).max(1 - b) <= k && k <= (n - a).min(n - b))
                || k == b - j && (1 - a).max(b - n) <= k && k <= (n - a).min(b - 1)
            {
                ans[(i - p) as usize][(j - r) as usize] = '#';
            }
        }
    }
    for ans in ans {
        println!("{}", ans.iter().join(""));
    }
}
