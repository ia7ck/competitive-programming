use least_prime_factors::least_prime_factors;
use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let (a, b) = scan!((usize, usize));
    let (c, d) = scan!((usize, usize));

    let lpf = least_prime_factors(201);
    let is_prime = |n: usize| lpf[n] == n;

    let mut takahashi = false;
    for x in a..=b {
        let mut aoki = false;
        for y in c..=d {
            if is_prime(x + y) {
                aoki = true;
            }
        }
        if !aoki {
            takahashi = true;
        }
    }
    if takahashi {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}
