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

    let (n, h) = scan!((usize, u64));
    let ab = scan!((u64, u64); n);

    let a_max = ab.iter().copied().map(|(a, _)| a).max().unwrap();
    let mut ab = ab;
    ab.sort_by_key(|&(_, b)| b);
    ab.reverse();
    let mut ans = h;
    let mut acc = 0;
    for (i, &(_, b)) in ab.iter().enumerate() {
        acc += b;
        let h = h.saturating_sub(acc);
        ans = ans.min((i + 1) as u64 + (h + a_max - 1) / a_max);
    }
    println!("{}", ans);
}
