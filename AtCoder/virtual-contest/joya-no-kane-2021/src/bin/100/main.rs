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

    let (n, m) = scan!((usize, usize));
    let a = scan!(i64; n);
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        let (x, y) = scan!((usize, usize));
        g[x - 1].push(y - 1);
    }
    let mut dp_min = vec![std::i64::MAX; n];
    for i in 0..n {
        for &j in &g[i] {
            dp_min[j] = dp_min[j].min(dp_min[i]).min(a[i]);
        }
    }
    let mut ans = std::i64::MIN;
    for i in 0..n {
        if dp_min[i] != std::i64::MAX {
            ans = ans.max(a[i] - dp_min[i]);
        }
    }
    assert_ne!(ans, std::i64::MIN);
    println!("{}", ans);
}
