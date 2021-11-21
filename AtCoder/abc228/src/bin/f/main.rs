use input_i_scanner::InputIScanner;
use std::collections::VecDeque;

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

    let (h, w, h1, w1, h2, w2) = scan!((usize, usize, usize, usize, usize, usize));
    let a: Vec<Vec<i64>> = (0..h).map(|_| scan!(i64; w)).collect();

    let h2 = h2.min(h1);
    let w2 = w2.min(w1);

    let mut b = a.clone();
    for i in 0..h {
        for j in 1..w {
            b[i][j] += b[i][j - 1];
        }
    }
    for j in 0..w {
        for i in 1..h {
            b[i][j] += b[i - 1][j];
        }
    }

    // c[i][j] := sum(a[i..i+h2][j..j+w2])
    let mut c = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            if i + h2 - 1 < h && j + w2 - 1 < w {
                c[i][j] = b[i + h2 - 1][j + w2 - 1];
                if i >= 1 {
                    c[i][j] -= b[i - 1][j + w2 - 1];
                }
                if j >= 1 {
                    c[i][j] -= b[i + h2 - 1][j - 1];
                }
                if i >= 1 && j >= 1 {
                    c[i][j] += b[i - 1][j - 1];
                }
            }
        }
    }

    // d[i][j] := max(c[i][j..j+(w1-w2)+1])
    let mut d = vec![vec![0; w]; h];
    for i in 0..h {
        let mut deq = VecDeque::new();
        let mut maxs = Vec::new();
        for j in 0..w {
            while !deq.is_empty() && c[i][deq.back().copied().unwrap()] <= c[i][j] {
                deq.pop_back();
            }
            deq.push_back(j);
            if j >= w1 - w2 {
                let k = deq.front().copied().unwrap();
                maxs.push(c[i][k]);
                if k == j - (w1 - w2) {
                    deq.pop_front();
                }
            }
        }
        for j in 0..maxs.len() {
            d[i][j] = maxs[j];
        }
    }

    // e[i][j] := max(d[i..i+(h1-h2)+1][j])
    let mut e = vec![vec![0; w]; h];
    for j in 0..w {
        let mut deq: VecDeque<usize> = VecDeque::new();
        let mut maxs = Vec::new();
        for i in 0..h {
            while !deq.is_empty() && d[deq.back().copied().unwrap()][j] <= d[i][j] {
                deq.pop_back();
            }
            deq.push_back(i);
            if i >= h1 - h2 {
                let k = deq.front().copied().unwrap();
                maxs.push(d[k][j]);
                if k == i - (h1 - h2) {
                    deq.pop_front();
                }
            }
        }
        for i in 0..maxs.len() {
            e[i][j] = maxs[i];
        }
    }

    // f[i][j] := sum(a[i..i+h1][j..j+w1])
    let mut f = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            if i + h1 - 1 < h && j + w1 - 1 < w {
                f[i][j] = b[i + h1 - 1][j + w1 - 1];
                if i >= 1 {
                    f[i][j] -= b[i - 1][j + w1 - 1];
                }
                if j >= 1 {
                    f[i][j] -= b[i + h1 - 1][j - 1];
                }
                if i >= 1 && j >= 1 {
                    f[i][j] += b[i - 1][j - 1];
                }
            }
        }
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            ans = ans.max(f[i][j] - e[i][j]);
        }
    }
    println!("{}", ans);
}
