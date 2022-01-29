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

    let mut ques = vec![VecDeque::new(); 26];
    for i in (0..n).rev() {
        let d = s[i] as usize - 'a' as usize;
        ques[d].push_back(i);
    }
    let mut pairs = Vec::new();
    let mut k = n;
    for i in 0..n {
        let d = s[i] as usize - 'a' as usize;
        for e in 0..d {
            let mut found = false;
            while let Some(j) = ques[e].pop_front() {
                if i < j && j < k {
                    pairs.push((i, j));
                    found = true;
                    k = j;
                    break;
                }
            }
            if found {
                break;
            }
        }
    }
    let mut s = s;
    for (i, j) in pairs {
        s.swap(i, j);
    }
    let ans = s.iter().join("");
    println!("{}", ans);
}

// cabaaabbbabcbaba
// ^              ^
// aabaaabbbabcbabc
//   ^          ^
// aaaaaabbbabcbbbc
//       ^  ^
// aaaaaaabbbbcbbbc