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
    let a = scan!(u64; n);

    let mut a = a;
    a.sort();
    let max = a.last().copied().unwrap();
    let mut x = a[0];
    for i in 1..(n - 1) {
        if a[i].max(max - a[i]) < x.max(max - x) {
            x = a[i];
        }
    }
    println!("{} {}", max, x);
}
