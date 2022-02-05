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

    let n = scan!(u64);
    let len = n.to_string().len();

    type Mint = ModInt998244353;
    let mut ans = Mint::new(0);
    ans += Mint::from(n + 1 + 1) * Mint::from(n + 1) / 2;
    for i in 0..(len - 1) {
        let ten = 10_u64.pow(i as u32);
        ans -= Mint::from(ten) * Mint::from(ten * 10 - ten);
    }
    let ten = 10_u64.pow((len - 1) as u32);
    ans -= Mint::from(ten) * Mint::from(n - ten + 1);
    println!("{}", (ans - 1).val());
}

// f(238) = 238-100+1
// f(237) = 237-100+1
// f(236) = 236-100+1
// ...
// f(100) = 100-100+1
// f(99)  = 99-10+1
// ...
// f(10)  = 10-10+1
// f(9)   = 9-1+1
// ...
// f(1)   = 1-1+1