use input_i_scanner::{scan_with, InputIScanner};

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let (a, b, c) = scan_with!(_i_i, (u32, u32, u32));

    for x in a..=b {
        if x % c == 0 {
            println!("{}", x);
            return;
        }
    }
    println!("-1");
}
