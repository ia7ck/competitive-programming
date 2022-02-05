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
    match n {
        1 => {
            // 2, 1
            println!("Yes");
        }
        2 => {
            // 4, 4
            println!("No");
        }
        3 => {
            // 8, 9
            println!("No");
        }
        4 => {
            // 16, 16
            println!("No");
        }
        _ => {
            println!("Yes");
        }
    }
}
