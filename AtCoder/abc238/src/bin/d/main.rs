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

    let t = scan!(usize);
    for _ in 0..t {
        let (a, s) = scan!((u64, u64));
        solve(a, s);
    }
}

fn solve(a: u64, s: u64) {
    if a * 2 > s {
        println!("No");
        return;
    }
    let mut t = a * 2;
    for i in (0..60).rev() {
        if a >> i & 1 == 1 {
            continue;
        }
        let p = 2_u64.pow(i as u32);
        if t + p <= s {
            t += p;
        }
    }
    if t == s {
        println!("Yes");
    } else {
        println!("No");
    }
}
