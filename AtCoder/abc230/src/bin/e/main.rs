use floor_sqrt::floor_sqrt;
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

    let n = scan!(u64);
    // for i in 1..=n {
    //     print!("{:3}", i);
    // }
    // println!();
    // for i in 1..=n {
    //     print!("{:3}", n / i);
    // }

    let s = floor_sqrt(n);
    let mut ans = 0;
    for i in 1..=s {
        ans += n / i;
    }
    let t = n / (s + 1);
    for i in 1..=t {
        let l = n / (i + 1);
        let r = n / i;
        ans += i * (r - l);
    }

    println!("{}", ans);
}

//   1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20 21 22 23
//  23 11  7  5  4  3  3  2  2  2  2  1  1  1  1  1  1  1  1  1  1  1  1

// 23/2 = 11
