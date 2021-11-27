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
    let k = scan!(usize);

    let mut ans = 0;
    let mut dot = 0;
    let mut j = 0;
    for (i, &ch) in s.iter().enumerate() {
        if ch == '.' {
            dot += 1;
        }
        while j < i && dot > k {
            if s[j] == '.' {
                dot -= 1;
            }
            j += 1;
        }
        // eprintln!("{} {} {}", j, i, dot);
        if dot <= k {
            ans = ans.max(i - j + 1);
        }
    }
    println!("{}", ans);
}
