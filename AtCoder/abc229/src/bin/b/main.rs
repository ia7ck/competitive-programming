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

    let a = scan!(String);
    let a: Vec<u32> = a.chars().map(|ch| ch as u32 - '0' as u32).collect();
    let b = scan!(String);
    let b: Vec<u32> = b.chars().map(|ch| ch as u32 - '0' as u32).collect();

    for (a, b) in a.into_iter().rev().zip(b.into_iter().rev()) {
        if a + b >= 10 {
            println!("Hard");
            return;
        }
    }
    println!("Easy");
}
