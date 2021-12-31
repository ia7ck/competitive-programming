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

    let s = scan!(String);
    let k = scan!(usize);

    let mut s: Vec<char> = s.chars().collect();
    let n = s.len();
    let mut k = k;
    for i in 0..n {
        if s[i] == 'a' {
            continue;
        }
        let j = 'z' as usize - s[i] as usize + 1;
        if j <= k {
            s[i] = 'a';
            k -= j;
        }
    }
    k %= 26;
    let last = s[n - 1];
    assert!(last as u8 + k as u8 <= 'z' as u8);
    s[n - 1] = (last as u8 + k as u8) as char;
    println!("{}", s.iter().join(""));
}
