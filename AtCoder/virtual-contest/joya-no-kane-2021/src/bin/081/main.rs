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

    let s = scan!(usize);

    macro_rules! add {
        ($a: expr, $b: expr) => {
            $a = ($a + $b) % 1_000_000_007_u64;
        };
    }

    let mut dp = vec![0; s + 1];
    dp[0] = 1;
    for sum in 0..s {
        for x in (3..=s).rev() {
            if sum + x <= s {
                add!(dp[sum + x], dp[sum]);
            }
        }
    }
    println!("{}", dp[s]);
}
