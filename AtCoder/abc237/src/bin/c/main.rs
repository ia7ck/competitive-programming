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

    let mut back = 0;
    for &ch in s.iter().rev() {
        if ch == 'a' {
            back += 1;
        } else {
            break;
        }
    }
    let mut front = 0;
    for &ch in s.iter() {
        if ch == 'a' {
            front += 1;
        } else {
            break;
        }
    }
    if front > back {
        println!("No");
        return;
    }
    let mut t = vec!['a'; back - front];
    for ch in s {
        t.push(ch);
    }
    let rev_t = {
        let mut rev_t = t.clone();
        rev_t.reverse();
        rev_t
    };
    if t == rev_t {
        println!("Yes");
    } else {
        println!("No");
    }
}
