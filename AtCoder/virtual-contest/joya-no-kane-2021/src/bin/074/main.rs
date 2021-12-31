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
    let s: Vec<char> = s.chars().collect();

    let mut alph = s.clone();
    alph.sort();
    alph.dedup();
    let mut ans = s.len();
    for target in alph {
        let mut s = s.clone();
        let mut count = 0;
        loop {
            let ok = s.iter().all(|&ch| ch == target);
            if ok {
                break;
            }
            count += 1;
            let t: Vec<char> = (0..(s.len() - 1))
                .map(|i| if s[i] == target { s[i] } else { s[i + 1] })
                .collect();
            s = t;
        }
        ans = ans.min(count);
    }
    println!("{}", ans);
}
