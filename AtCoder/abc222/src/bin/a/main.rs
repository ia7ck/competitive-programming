use input_i_scanner::{scan_with, InputIScanner};

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let n = scan_with!(_i_i, usize);
    if n < 10 {
        print!("000");
    } else if n < 100 {
        print!("00");
    } else if n < 1000 {
        print!("0");
    } else {
        //
    }
    println!("{}", n);
}
