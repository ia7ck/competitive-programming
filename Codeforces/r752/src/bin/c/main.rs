use input_i_scanner::{scan_with, InputIScanner};

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let t = scan_with!(_i_i, usize);
    for _ in 0..t {
        let n = scan_with!(_i_i, usize);
        let a = scan_with!(_i_i, u64; n);
        solve(n, a);
    }
}

fn solve(n: usize, a: Vec<u64>) {
    let mut l = 1;
    for i in 0..n {
        l = lcm(l, (i + 2) as u64);
        if l > 1_000_000_000 {
            break;
        }
        if a[i] % l == 0 {
            println!("NO");
            return;
        }
    }
    println!("YES");
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}
