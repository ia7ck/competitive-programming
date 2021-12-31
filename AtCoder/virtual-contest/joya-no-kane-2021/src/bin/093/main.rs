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

    let _n = scan!(usize);
    let s = scan!(String);
    let t = scan!(String);

    let mut s_zeros = Vec::new();
    let mut t_zeros = Vec::new();
    for (i, ch) in s.chars().enumerate() {
        if ch == '0' {
            s_zeros.push(i);
        }
    }
    for (i, ch) in t.chars().enumerate() {
        if ch == '0' {
            t_zeros.push(i);
        }
    }
    if s_zeros.len() != t_zeros.len() {
        println!("-1");
        return;
    }
    let mut ans = 0;
    for (i, j) in s_zeros.iter().zip(t_zeros.iter()) {
        if i != j {
            ans += 1;
        }
    }
    println!("{}", ans);
}
