use coordinate_compression::CoordinateCompression;
use input_i_scanner::{scan_with, InputIScanner};
use std::iter::FromIterator;

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let (h, w, n) = scan_with!(_i_i, (usize, usize, usize));
    let mut r_c_a = Vec::new();
    let mut a = Vec::new();
    for _ in 0..n {
        let (r, c, aa) = scan_with!(_i_i, (usize, usize, u32));
        r_c_a.push((r, c, aa));
        a.push(aa);
    }

    let cmp = CoordinateCompression::from_iter(a.iter().copied());
    let mut r_c_i = vec![vec![]; n];
    for (i, &(r, c, a)) in r_c_a.iter().enumerate() {
        r_c_i[cmp.find_index(&a)].push((r, c, i));
    }
    // eprintln!("{:?}", rc);
    let mut r_max = vec![-1; h + 1];
    let mut c_max = vec![-1; w + 1];
    let mut ans = vec![0; n];
    for a in (0..n).rev() {
        for &(r, c, i) in &r_c_i[a] {
            ans[i] = (r_max[r] + 1).max(c_max[c] + 1);
        }
        for &(r, c, i) in &r_c_i[a] {
            r_max[r] = r_max[r].max(ans[i]);
            c_max[c] = c_max[c].max(ans[i]);
        }
    }
    for ans in ans {
        println!("{}", ans);
    }
}
