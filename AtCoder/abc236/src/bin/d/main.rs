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
    let mut a = vec![vec![]; n * 2];
    for i in 0..(n * 2 - 1) {
        a[i] = scan!(u32; n * 2 - i - 1);
    }
    let m = 1 << (n * 2);
    let mut ans = 0;
    solve(n, m - 1, &a, 0, &mut ans);
    println!("{}", ans);
}

fn solve(n: usize, bits: usize, a: &Vec<Vec<u32>>, acc: u32, ans: &mut u32) {
    if bits == 0 {
        if acc > *ans {
            *ans = acc;
        }
        return;
    }
    for i in 0..(n * 2) {
        if bits >> i & 1 == 1 {
            for j in (i + 1)..(n * 2) {
                if bits >> j & 1 == 1 {
                    solve(n, bits ^ (1 << i) ^ (1 << j), a, acc ^ a[i][j - i - 1], ans);
                }
            }
            break;
        }
    }
}
