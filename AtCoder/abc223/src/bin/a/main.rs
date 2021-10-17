use input_i_scanner::{InputIScanner, scan_with};

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let x = scan_with!(_i_i, usize);
    if x > 0 && x % 100 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
