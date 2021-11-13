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

    let (n, k) = scan!((usize, u64));
    let mut a = scan!(u64; n);
    a.sort();

    let f = |proj_num: u64| -> bool {
        let h = a.iter().filter(|&&a| a >= proj_num).count() as u64;
        if h >= k {
            return true;
        }
        let sum = a.iter().filter(|&&a| a < proj_num).sum::<u64>();
        sum >= proj_num * (k - h)
    };

    let mut ok = 0;
    let mut ng = 1_000_000_000_000_000_000;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        if f(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}
