use std::collections::HashSet;

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

    let s = scan!(String);
    let t = scan!(String);

    let mut s: Vec<char> = s.chars().collect();
    let mut t: Vec<char> = t.chars().collect();

    for _ in 0..2 {
        let mut dest = vec![HashSet::new(); 26];
        for i in 0..s.len() {
            dest[s[i] as usize - 'a' as usize].insert(t[i] as usize - 'a' as usize);
        }
        let ok = dest.iter().all(|s| s.len() <= 1);
        if !ok {
            println!("No");
            return;
        }
        std::mem::swap(&mut s, &mut t);
    }

    println!("Yes");
}
