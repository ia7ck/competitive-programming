use input_i_scanner::{scan_with, InputIScanner};

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let n = scan_with!(_i_i, String);
    let n: Vec<char> = n.chars().collect();

    let f = |x: &[u64]| -> u64 {
        let mut res = 0;
        for d in x {
            res *= 10;
            res += d;
        }
        res
    };

    let mut ans = 0;
    for bits in 0..(1 << n.len()) {
        let mut x = Vec::new();
        let mut y = Vec::new();
        for i in 0..n.len() {
            if bits >> i & 1 == 1 {
                x.push(n[i] as u64 - '0' as u64);
            } else {
                y.push(n[i] as u64 - '0' as u64);
            }
        }
        x.sort();
        x.reverse();
        y.sort();
        y.reverse();
        let x = f(&x);
        let y = f(&y);
        if x > 0 && y > 0 {
            ans = ans.max(x * y);
        }
    }
    assert!(ans > 0);
    println!("{}", ans);
}
