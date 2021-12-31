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

    let (n, m, q) = scan!((usize, usize, usize));
    let mut wv = scan!((u64, u64); n);
    wv.sort_by_key(|&(_, v)| v);
    wv.reverse();
    let x = scan!(u64; m);
    for _ in 0..q {
        let (l, r) = scan!((usize, usize));
        let mut x: Vec<u64> = x
            .iter()
            .copied()
            .take(l - 1)
            .chain(x.iter().copied().skip(r).take(m - r))
            .collect();
        x.sort();
        let mut ans = 0;
        for &(w, v) in &wv {
            if let Some(i) = x.iter().position(|&x| x >= w) {
                ans += v;
                x.remove(i);
            }
        }
        println!("{}", ans);
    }
}
