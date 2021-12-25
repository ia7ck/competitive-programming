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

    let (l, r) = scan!((usize, usize));
    let s = scan!(String);
    let s: Vec<char> = s.chars().collect();

    let mut t = String::new();
    for &ch in &s[..(l - 1)] {
        t.push(ch);
    }
    for &ch in s[(l - 1)..r].iter().rev() {
        t.push(ch);
    }
    for &ch in &s[r..] {
        t.push(ch);
    }
    println!("{}", t);
}
