use binary_search::BinarySearch;
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

    let (n, m) = scan!((usize, usize));
    let h = scan!(u64; n);
    let w = scan!(u64; m);

    let mut h = h;
    h.sort();
    let mut left = vec![0; n + 1];
    for i in (2..=n).step_by(2) {
        left[i] = left[i - 2] + (h[i - 1] - h[i - 2]);
    }
    let mut right = vec![0; n + 1];
    for i in (1..(n - 1)).rev() {
        right[i] = right[i + 2] + (h[i + 1] - h[i]);
    }
    let mut ans = std::u64::MAX;
    for w in w {
        let i = h.lower_bound(&w);
        if i == 0 {
            ans = ans.min((h[0] - w) + right[1]);
        } else if i == n {
            ans = ans.min((w - h[n - 1]) + left[n - 1]);
        } else if i % 2 == 1 {
            ans = ans.min(absolute(w, h[i - 1]) + left[i - 1] + right[i]);
        } else {
            ans = ans.min(absolute(w, h[i]) + left[i] + right[i + 1]);
        }
    }
    println!("{}", ans);
}

fn absolute(a: u64, b: u64) -> u64 {
    a.max(b) - a.min(b)
}
