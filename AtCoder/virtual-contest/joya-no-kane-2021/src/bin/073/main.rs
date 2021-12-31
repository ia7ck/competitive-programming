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
    let a = scan!(u32; n);

    if n % 2 == 1 {
        println!("Win");
        return;
    }
    let xor = a.iter().fold(0, |acc, x| acc ^ x);
    let win = a.iter().any(|&x| x == xor);
    if win {
        println!("Win");
    } else {
        println!("Lose");
    }
}
