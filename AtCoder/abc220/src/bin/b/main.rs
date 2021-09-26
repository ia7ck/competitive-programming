use input_i_scanner::{scan_with, InputIScanner};

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let k = scan_with!(_i_i, u64);
    let (a, b) = scan_with!(_i_i, (String, String));

    let f = |v: &[u64]| -> u64 {
        let mut res = 0;
        let mut o = 1;
        for v in v.iter().rev() {
            res += v * o;
            o *= k;
        }
        res
    };

    let a: Vec<u64> = a.chars().map(|c| c as u64 - '0' as u64).collect();
    let b: Vec<u64> = b.chars().map(|c| c as u64 - '0' as u64).collect();

    let a = f(&a);
    let b = f(&b);
    println!("{}", a * b);
}
