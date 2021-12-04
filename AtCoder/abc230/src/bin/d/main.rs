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

    let (n, d) = scan!((usize, u32));
    let mut lr = scan!((u32, u32); n);

    lr.sort_by_key(|&(_, r)| r);
    let mut x = 0;
    let mut ans = 0;
    for (l, r) in lr {
        if x < l {
            ans += 1;
            x = r + d - 1;
        }
    }
    println!("{}", ans);
}
