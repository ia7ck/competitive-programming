use input_i_scanner;
use input_i_scanner::InputIScanner;
use prime_factorization::PrimeFactorization;

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let k = input_i_scanner::scan_with!(_i_i, u64);
    let p_e = k.prime_factorization();

    let mut n = 1u128;
    for (p, e) in p_e {
        let p = p as u128;
        n *= p.pow(((e + 1) / 2) as u32);
    }
    let k = k as u128;
    if n * n % k == 0 && n % k != 0 {
        println!("{}", n);
    } else {
        println!("-1");
    }
}
