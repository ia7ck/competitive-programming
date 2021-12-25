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

    let x = scan!(String);
    let x: Vec<u64> = x.chars().map(|ch| ch as u64 - '0' as u64).collect();

    let mut ans = Vec::new();
    let mut s = x.iter().sum::<u64>();
    let mut carry = 0;
    for &x in x.iter().rev() {
        ans.push((s + carry) % 10);
        carry = (s + carry) / 10;
        s -= x;
    }
    if carry > 0 {
        assert_eq!(carry, 1);
        ans.push(1);
    }
    ans.reverse();
    println!("{}", ans.iter().join(""));
}
