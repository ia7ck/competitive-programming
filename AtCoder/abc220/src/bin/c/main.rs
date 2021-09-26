use input_i_scanner::{scan_with, InputIScanner};

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let n = scan_with!(_i_i, usize);
    let a = scan_with!(_i_i, u64; n);
    let x = scan_with!(_i_i, u64);

    let s = a.iter().copied().sum::<u64>();
    let c = x / s;
    let mut s = c * s;
    for i in 0..n {
        s += a[i];
        if s > x {
            println!("{}", (c as usize * n) + (i + 1));
            return;
        }
    }
    unreachable!();
}
