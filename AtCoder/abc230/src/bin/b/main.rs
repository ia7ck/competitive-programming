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

    let t: Vec<char> = "oxx".repeat(20).chars().collect();
    for i in 0..t.len() {
        if i + s.len() <= t.len() {
            let mut ok = true;
            for j in 0..s.len() {
                if s[j] != t[i + j] {
                    ok = false;
                    break;
                }
            }
            if ok {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
