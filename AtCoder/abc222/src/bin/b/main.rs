use input_i_scanner::{scan_with, InputIScanner};

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let n = scan_with!(_i_i, usize);
    let p = scan_with!(_i_i, u32);
    let a = scan_with!(_i_i, u32; n);

    let ans = a.into_iter().filter(|&a| a < p).count();
    println!("{}", ans);
}
