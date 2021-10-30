use input_i_scanner::{scan_with, InputIScanner};

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let (n, m) = scan_with!(_i_i, (usize, usize));
    let mut b = vec![vec![]; n];
    for i in 0..n {
        b[i] = scan_with!(_i_i, u64; m);
    }

    let ok = (0..n).all(|i| (1..m).all(|j| b[i][j - 1] + 1 == b[i][j]));
    if !ok {
        println!("No");
        return;
    }
    let ok = (0..m).all(|j| (1..n).all(|i| b[i - 1][j] + 7 == b[i][j]));
    if !ok {
        println!("No");
        return;
    }
    let ok = (0..n).all(|i| (0..(m - 1)).all(|j| b[i][j] % 7 != 0));
    if !ok {
        println!("No");
        return;
    }
    
    println!("Yes");
}

// 2 4
// 6 7 8 9
// 13 14 15 16
