use factorials::Factorial;
use input_i_scanner::InputIScanner;
use mod_int::ModInt1000000007;

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

    type Mint = ModInt1000000007;

    let (x, y) = scan!((usize, usize));

    let f = Factorial::new(x + y, 1_000_000_000 + 7);
    let mut ans = Mint::new(0);
    for i in 0..=x {
        // x - i, y - i * 2
        if y < i * 2 {
            continue;
        }
        if (x - i) % 2 != 0 {
            continue;
        }
        // (+1, +2): i
        // (+2, +1): (x - i) / 2
        if i * 2 + (x - i) / 2 != y {
            continue;
        }
        ans += f.binomial(i + (x - i) / 2, i);
    }
    println!("{}", ans.val());
}
