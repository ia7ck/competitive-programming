use input_i_scanner::{scan_with, InputIScanner};

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let (h, w) = scan_with!(_i_i, (usize, usize));
    let mut a = vec![vec![]; h];
    for i in 0..h {
        a[i] = scan_with!(_i_i, u64; w);
    }

    let mut ok = true;
    for i1 in 0..h {
        for i2 in (i1 + 1)..h {
            for j1 in 0..w {
                for j2 in (j1 + 1)..w {
                    if a[i1][j1] + a[i2][j2] > a[i2][j1] + a[i1][j2] {
                        ok = false;
                    }
                }
            }
        }
    }
    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
