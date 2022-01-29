use input_i_scanner::InputIScanner;
use mod_int::ModInt998244353;

type Mint = ModInt998244353;

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
    let a = scan!(u64; n);

    let tot = a.iter().sum::<u64>();
    if a[0] <= tot - a[0] {
        println!("0");
        return;
    }

    let mut ans = Mint::new(1);
    for i in 1..n {
        ans *= binom(a[i] + k - 1, k - 1);
    }
    ans *= binom(a[0] - (tot - a[0]) - 1, k - 1);
    println!("{}", ans.val());
}

fn binom(n: u64, k: u64) -> Mint {
    if n < k {
        return Mint::new(0);
    }
    let mut numer = Mint::new(1);
    for i in 0..k {
        numer *= Mint::from(n - i);
    }
    let mut denom = Mint::new(1);
    for i in 1..=k {
        denom *= Mint::from(i);
    }
    numer / denom
}