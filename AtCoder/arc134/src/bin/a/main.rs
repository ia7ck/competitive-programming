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

    let (n, l, w) = scan!((usize, i64, i64));
    let a = scan!(i64; n);

    let mut ans = 0;
    if a[0] != 0 {
        ans += (a[0] + (w - 1)) / w;
    }
    let mut cur = a[0] + w;
    for i in 1..n {
        if cur < a[i] {
            let c = (a[i] - cur + (w - 1)) / w; 
            ans += c;
            cur = a[i] + w;
        } else {
            cur = a[i] + w;
        }
    }
    if cur < l {
        ans += (l - cur + (w - 1)) / w;
    }
    println!("{}", ans);
}
