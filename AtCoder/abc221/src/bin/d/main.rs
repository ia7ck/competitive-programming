use coordinate_compression::CoordinateCompression;
use input_i_scanner::{scan_with, InputIScanner};
use join::Join;

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let n = scan_with!(_i_i, usize);
    let ab = scan_with!(_i_i, (usize, usize); n);

    let mut values = Vec::new();
    for &(a, b) in &ab {
        values.push(a - 1);
        values.push(a - 1 + b);
    }
    let values: CoordinateCompression<usize> = values.into_iter().collect();
    let m = 4_000_00;
    let mut imos = vec![0i64; m + 1];
    for (a, b) in &ab {
        imos[values.find_index(&(a - 1))] += 1;
        imos[values.find_index(&(a - 1 + b))] -= 1;
    }
    for i in 0..m {
        imos[i + 1] += imos[i];
    }
    let mut ans = vec![0; n + 1];
    for i in 0..m {
        if imos[i] > 0 {
            let l = values[i];
            let r = values[i + 1];
            ans[imos[i] as usize] += r - l;
        }
    }
    println!("{}", ans[1..].iter().join(" "));
}
