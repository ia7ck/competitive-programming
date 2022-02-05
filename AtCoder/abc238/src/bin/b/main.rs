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

    let n = scan!(usize);
    let a = scan!(usize; n);

    let mut cut = vec![false; 360];
    cut[0] = true;
    let mut deg = 0;
    for a in a {
        deg += a;
        deg %= 360;
        assert_eq!(cut[deg], false);
        cut[deg] = true;
    }
    let cut: Vec<bool> = cut.iter().copied().chain(cut.iter().copied()).collect();
    let mut ans = 0;
    let mut prev = 0;
    for i in 1..cut.len() {
        if cut[i] {
            ans = ans.max(i - prev);
            prev = i;
        }
    }
    assert_ne!(ans, 0);
    println!("{}", ans);
}
