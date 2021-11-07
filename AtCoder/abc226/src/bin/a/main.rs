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

    let x = scan!(String);
    let x: Vec<&str> = x.split('.').collect();
    let a: u32 = x[0].parse().unwrap();
    let x: Vec<char> = x[1].chars().collect();
    if x[0] < '5' {
        println!("{}", a);
    } else {
        println!("{}", a + 1);
    }
}
