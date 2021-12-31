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

    let (n, k) = scan!((usize, usize));
    let mut ans = 0;
    for a in 1..=n {
        if a % k == 0 {
            // b % k == 0, c % k == 0
            ans += (n / k) * (n / k);
        } else {
            // b % k == k - a % k, c % k == k - a % k
            // (b + c) % k == 0
            let j = k - a % k;
            if (j + j) % k == 0 {
                if n >= j {
                    ans += ((n - j) / k + 1) * ((n - j) / k + 1);
                }
            }
        }
    }
    println!("{}", ans);
}
