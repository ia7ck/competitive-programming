use input_i_scanner::InputIScanner;
use std::collections::BTreeSet;

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

    const N: usize = 1usize << 20;
    let mut a = vec![-1i64; N];
    let mut set = BTreeSet::new();
    for i in 0..N * 2 {
        set.insert(i);
    }
    let q = scan!(usize);
    for _ in 0..q {
        let (t, x) = scan!((u8, usize));
        match t {
            1 => {
                let i = set.range((x % N)..).next().copied().unwrap();
                if i < N {
                    a[i] = x as i64;
                    set.remove(&i);
                    set.remove(&(i + N));
                } else {
                    a[i - N] = x as i64;
                    set.remove(&i);
                    set.remove(&(i - N));
                }
            }
            2 => {
                println!("{}", a[x % N]);
            }
            _ => unreachable!(),
        }
    }
}
