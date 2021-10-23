use input_i_scanner::{scan_with, InputIScanner};

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let n = scan_with!(_i_i, usize);
    let points = scan_with!(_i_i, (i64, i64); n);

    let mut ans = 0_u64;
    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                let (xi, yi) = points[i];
                let (xj, yj) = points[j];
                let (xk, yk) = points[k];
                let (dx_ji, dy_ji) = (xj - xi, yj - yi);
                let (dx_ki, dy_ki) = (xk - xi, yk - yi);
                if dy_ji * dx_ki == dy_ki * dx_ji {
                    //
                } else {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
