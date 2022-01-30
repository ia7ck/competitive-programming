use std::collections::VecDeque;

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

    let n = scan!(usize);
    let s = scan!(String);
    let s: Vec<char> = s.chars().collect();
    
    let mut a = VecDeque::new();
    a.push_back(n);
    for i in (0..n).rev() {
        if s[i] == 'L' {
            a.push_back(i);
        } else {
            a.push_front(i);
        }
    }
    let ans = a.iter().join(" ");
    println!("{}", ans);
}
