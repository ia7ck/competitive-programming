use input_i_scanner::{scan_with, InputIScanner};

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let n = scan_with!(_i_i, usize);
    let mut deg = vec![0; n];
    for _ in 0..(n - 1) {
        let (a, b) = scan_with!(_i_i, (usize, usize));
        deg[a - 1] += 1;
        deg[b - 1] += 1;
    }
    if deg.contains(&(n - 1)) {
        println!("Yes");
    } else {
        println!("No");
    }
}
