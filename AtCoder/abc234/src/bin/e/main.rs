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

    let x = scan!(String);
    let x: Vec<i64> = x.chars().map(|ch| ch as i64 - '0' as i64).collect();

    let n = x.len();
    let top = x[0];
    let mut ans = vec![9; n];

    for diff in (-9)..=9 {
        for start in top..=(top + 1).min(9) {
            let mut y = Vec::new();
            for i in 0..n {
                y.push(start + diff * i as i64);
            }
            let ok = y.iter().all(|&d| 0 <= d && d <= 9);
            if ok && y >= x {
                ans = ans.min(y);
            }
        }
    }

    println!("{}", ans.iter().join(""));
}
