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
    let ss = scan!(u64; n);

    let mut ans = 0;
    for s in ss {
        let mut ok = false;
        for a in 1..=s {
            for b in 1..=s {
                if 4 * a * b + 3 * a + 3 * b == s {
                    ok = true;
                }
            }
        }
        if !ok {
            ans += 1;
        }
    }
    println!("{}", ans);
}
