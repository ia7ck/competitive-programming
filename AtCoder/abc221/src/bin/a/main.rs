use input_i_scanner::{InputIScanner, scan_with};

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let (a, b) = scan_with!(_i_i, (u32, u32));

    let ans = 32_u64.pow(a - b);
    println!("{}", ans);
}
