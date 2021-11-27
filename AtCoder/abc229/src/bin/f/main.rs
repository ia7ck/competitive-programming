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
    let a = scan!(u64; n);
    let b = scan!(u64; n);

    let mut rr = 0u64;
    let mut rg = a[1] + b[0];
    let mut gr = a[0] + b[0];
    let mut gg = a[0] + a[1];
    for i in 3..=n {
        let (next_rr, next_rg, next_gr, next_gg) = (
            rr.max(rg + b[i - 2]),
            (rr + a[i - 1] + b[i - 2]).max(rg + a[i - 1]),
            gr.max(gg + b[i - 2]),
            (gr + a[i - 1] + b[i - 2]).max(gg + a[i - 1]),
        );
        rr = next_rr;
        rg = next_rg;
        gr = next_gr;
        gg = next_gg;
    }
    let total = a.iter().chain(b.iter()).sum::<u64>();
    let used = rr.max(rg + b[n - 1]).max(gr + b[n - 1]).max(gg);
    let ans = total - used;
    println!("{}", ans);
}
