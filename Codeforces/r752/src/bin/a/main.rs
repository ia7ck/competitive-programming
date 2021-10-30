use input_i_scanner::{scan_with, InputIScanner};

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let t = scan_with!(_i_i, usize);
    for _ in 0..t {
        let n = scan_with!(_i_i, usize);
        let a = scan_with!(_i_i, u64; n);
        solve(n, a);
    }
}

fn solve(n: usize, a: Vec<u64>) {
    let mut ans = 0;
    for i in 0..n {
        ans = ans.max(a[i].saturating_sub((i + 1) as u64));
    }
    println!("{}", ans);
}
