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

    let (n, p) = scan!((usize, u8));
    let a = scan!(u32; n);

    let odd = a.iter().filter(|&&x| x % 2 == 1).count();
    let even = n - odd;
    if odd == 0 && p == 1 {
        println!("0");
        return;
    }
    let ans = 2_u64.pow(even as u32) * 2_u64.pow(odd.saturating_sub(1) as u32);
    println!("{}", ans);
}
