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
    let rows = scan!(usize; n);
    let cols = scan!(usize; n);
    let q = scan!(usize);
    let mut ans = String::new();
    for _ in 0..q {
        let (r, c) = scan!((usize, usize));
        let (r, c) = (rows[r - 1], cols[c - 1]);
        if n - r < c {
            ans.push('#');
        } else {
            ans.push('.');
        }
    }
    println!("{}", ans);
}
