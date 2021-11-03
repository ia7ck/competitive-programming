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
        let n = scan!(usize);
        let a = scan!(i64; n);
        let s = scan!(String);
        solve(n, a, s);
    }
}

fn solve(_: usize, a: Vec<i64>, s: String) {
    let mut red = Vec::new();
    let mut blue = Vec::new();
    for (i, &c) in s.as_bytes().iter().enumerate() {
        if c == b'R' {
            red.push(a[i]);
        } else {
            blue.push(a[i]);
        }
    }
    red.sort();
    blue.sort();
    for i in 0..red.len() {
        if red[i] > (blue.len() + i + 1) as i64 {
            println!("NO");
            return;
        }
    }
    for i in 0..blue.len() {
        if blue[i] < (i + 1) as i64 {
            println!("NO");
            return;
        }
    }
    println!("YES");
}
