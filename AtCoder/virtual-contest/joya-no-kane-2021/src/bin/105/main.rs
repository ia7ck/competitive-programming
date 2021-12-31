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

    let (n, k) = scan!((usize, usize));
    let s = scan!(String);

    let s: Vec<char> = s.chars().collect();
    let mut runs = Vec::new();
    let mut zero = s[0] == '0';
    let mut last = s[0];
    let mut len = 1;
    for i in 1..n {
        if last != s[i] {
            runs.push((zero, len));
            zero = s[i] == '0';
            last = s[i];
            len = 1;
        } else {
            len += 1;
        }
    }
    runs.push((zero, len));

    let mut i = 0;
    let mut ans = 0;
    let mut sum = 0;
    let mut zero_count = 0;
    for &(zero, len) in &runs {
        sum += len;
        if zero {
            zero_count += 1;
        }
        while zero_count > k {
            let (zero, len) = runs[i];
            sum -= len;
            if zero {
                zero_count -= 1;
            }
            i += 1;
        }
        ans = ans.max(sum);
    }
    println!("{}", ans);
}
