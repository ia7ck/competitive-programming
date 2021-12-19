use input_i_scanner::InputIScanner;
use next_permutation::NextPermutation;

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
    let mut g1 = vec![vec![false; n]; n];
    for _ in 0..m {
        let (a, b) = scan!((usize, usize));
        g1[a - 1][b - 1] = true;
        g1[b - 1][a - 1] = true;
    }
    let mut g2 = vec![vec![false; n]; n];
    for _ in 0..m {
        let (c, d) = scan!((usize, usize));
        g2[c - 1][d - 1] = true;
        g2[d - 1][c - 1] = true;
    }

    let mut p: Vec<usize> = (0..n).collect();
    loop {
        let mut ok = true;
        for i in 0..n {
            for j in (i + 1)..n {
                if g1[i][j] != g2[p[i]][p[j]] {
                    ok = false;
                    break;
                }
            }
        }
        if ok {
            println!("Yes");
            return;
        }
        if !p.next_permutation() {
            break;
        }
    }
    println!("No");
}
