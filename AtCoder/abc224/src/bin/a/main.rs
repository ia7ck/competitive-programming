use input_i_scanner::{scan_with, InputIScanner};

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let s = scan_with!(_i_i, String);
    if s.ends_with("er") {
        println!("er");
    } else {
        println!("ist");
    }
}
