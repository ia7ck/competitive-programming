use input_i_scanner::InputIScanner;
use mod_int::ModInt998244353;

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

    type Mint = ModInt998244353;

    let n = scan!(usize);
    let a = scan!(u64; n);

    let mut a = a;
    a.sort();
    a.reverse();
    //   a[0] * (a[0] + a[1] + a[2] * 2 + a[3] * 4 + ...)
    // + a[1] * (a[1] + a[2] + a[3] * 2 + ...)
    let mut tmp = Mint::new(0);
    for i in 1..n {
        tmp += Mint::from(a[i]) * Mint::new(2).pow(i - 1);
    }
    let mut ans = Mint::new(0);
    for i in 0..n {
        let x = Mint::from(a[i]);
        ans += x * (x + tmp);
        if i + 1 < n {
            tmp -= a[i + 1];
            tmp /= 2;
        }
    }
    println!("{}", ans.val());
}
