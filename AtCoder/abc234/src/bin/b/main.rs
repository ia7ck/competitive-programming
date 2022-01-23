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
    let xy = scan!((f64, f64); n);

    let mut ans = 0_f64;
    for i in 0..n {
        for j in 0..i {
            let (x, y) = xy[i];
            let (xx, yy) = xy[j];
            ans = ans.max((x - xx).hypot(y - yy));
        }
    }
    println!("{}", ans);
}
