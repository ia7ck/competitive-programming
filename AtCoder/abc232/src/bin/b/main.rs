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
    let t = scan!(String);
    let t: Vec<char> = t.chars().collect();

    let dist = |c: char, d: char| -> u32 {
        let mut c = c as u32 - 'a' as u32;
        let d = d as u32 - 'a' as u32;
        let mut k = 0;
        while c != d {
            if c == 25 {
                c = 0;
            } else {
                c += 1;
            }
            k += 1;
        }
        k
    };

    let k = dist(s[0], t[0]);
    for i in 1..s.len() {
        if k != dist(s[i], t[i]) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
