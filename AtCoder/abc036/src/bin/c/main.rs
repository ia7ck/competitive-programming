use coordinate_compression::CoordinateCompression;
use input_i_scanner::{scan_with, InputIScanner};

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let n = scan_with!(_i_i, usize);
    let a = scan_with!(_i_i, u32; n);

    let cc: CoordinateCompression<u32> = a.iter().copied().collect();
    for a in a {
        println!("{}", cc.find_index(&a));
    }
}
