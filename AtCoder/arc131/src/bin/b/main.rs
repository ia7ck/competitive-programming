use grid_search::around;
use input_i_scanner::InputIScanner;
use join::Join;

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

    let (h, _w) = scan!((usize, usize));
    let mut a: Vec<Vec<char>> = vec![vec![]; h];
    for i in 0..h {
        let row = scan!(String);
        a[i] = row.chars().collect();
    }

    let ok = dfs(0, 0, &mut a);
    assert!(ok);
    for row in a {
        println!("{}", row.iter().join(""));
    }
}

const DIR: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn dfs(i: usize, j: usize, a: &mut Vec<Vec<char>>) -> bool {
    let h = a.len();
    let w = a[0].len();
    if i == h {
        return true;
    }
    let (ni, nj) = if j + 1 < w { (i, j + 1) } else { (i + 1, 0) };
    if a[i][j] != '.' {
        return dfs(ni, nj, a);
    }
    for d in b'1'..=b'5' {
        let d = d as char;
        let mut dup = false;
        for (y, x) in around(i, j).y_range(0..h).x_range(0..w).directions(&DIR) {
            if a[y][x] == d {
                dup = true;
                break;
            }
        }
        if dup {
            continue;
        }
        a[i][j] = d;
        if dfs(ni, nj, a) {
            return true;
        }
        a[i][j] = '.';
    }
    false
}
