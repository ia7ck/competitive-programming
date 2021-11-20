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

    let (n, x) = scan!((usize, usize));
    let a = scan!(usize; n);

    let mut seen = vec![false; n];
    let mut cur = x - 1;
    seen[cur] = true;
    loop {
        cur = a[cur] - 1;
        if seen[cur] {
            break;
        }
        seen[cur] = true;
    }
    let mut ans = 0;
    for i in 0..n {
        if seen[i] {
            ans += 1;
        }
    }
    println!("{}", ans);
}
