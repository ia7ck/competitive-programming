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

    let n = scan!(usize);
    let a = scan!(usize; n);

    let mut freq = vec![0; n];
    for &x in &a {
        freq[x] += 1;
    }
    if n % 2 == 0 {
        // n = 6
        // 0 1 2 3 4 5
        // 0 2 0 2 0 2
        for i in (1..n).step_by(2) {
            if freq[i] != 2 {
                println!("0");
                return;
            }
        }
    } else {
        // n = 7
        // 0 1 2 3 4 5 6
        // 1 0 2 0 2 0 2
        if freq[0] != 1 {
            println!("0");
            return;
        }
        for i in (2..n).step_by(2) {
            if freq[i] != 2 {
                println!("0");
                return;
            }
        }
    }
    let mut ans = 1;
    for _ in 0..n / 2 {
        ans = ans * 2 % 1_000_000_007;
    }
    println!("{}", ans);
}
