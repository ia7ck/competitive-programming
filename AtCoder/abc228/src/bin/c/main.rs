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
    let m = 1200;
    let mut p = Vec::new();
    let mut a = vec![0usize; m + 1];
    for _ in 0..n {
        let (p1, p2, p3) = scan!((usize, usize, usize));
        p.push(p1 + p2 + p3);
        a[p1 + p2 + p3] += 1;
    }

    for i in 0..m {
        a[i + 1] += a[i];
    }
    for p in p {
        if a[m] - a[p + 300] < k {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
