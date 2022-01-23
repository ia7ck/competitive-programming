use input_i_scanner::InputIScanner;
use join::Join;

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

    let mut k = scan!(usize);

    let mut ans = Vec::new();
    while k > 0 {
        ans.push(k % 2);
        k /= 2;
    }
    ans.reverse();
    let ans = ans
        .iter()
        .map(|d| match d {
            0 => 0,
            1 => 2,
            _ => unreachable!(),
        })
        .join("");
    println!("{}", ans);
}

// 2    1
// 20   2
// 22   3
// 200  4
// 202  5
// 220  6
// 222  7
// 2000 8
// 2002 9
// 2020 10
// 2022 11
// 2200 12
// 2202 13
// 2222 14
