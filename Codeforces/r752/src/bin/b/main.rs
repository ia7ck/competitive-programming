use input_i_scanner::{scan_with, InputIScanner};

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let t = scan_with!(_i_i, usize);
    for _ in 0..t {
        let n = scan_with!(_i_i, usize);
        let a = scan_with!(_i_i, u32; n);
        solve(n, a);
    }
}

fn solve(n: usize, a: Vec<u32>) {
    if n % 2 == 0 {
        println!("YES");
        return;
    }
    let ok = a.windows(2).any(|w| w[0] >= w[1]);
    if ok {
        println!("YES");
    } else {
        println!("NO");
    }
}
