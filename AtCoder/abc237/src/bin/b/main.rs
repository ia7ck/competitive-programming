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

    let (h, w) = scan!((usize, usize));
    let a: Vec<Vec<u32>> = (0..h).map(|_| scan!(u32; w)).collect();

    let mut b = vec![vec![0; h]; w];
    for i in 0..h {
        for j in 0..w {
            b[j][i] = a[i][j];
        }
    }

    for row in b {
        println!("{}", row.iter().join(" "));
    }
}
