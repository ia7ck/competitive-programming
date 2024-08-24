use proconio::input;
use prime_factorization::PrimeFactorization;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };

    let mut g = 0;
    for a in a {
        let mut c = 0;
        for (_, e) in a.prime_factorization() {
            c += e;
        }
        g ^= c;
    }

    if g == 0 {
        println!("Bruno");
    } else {
        println!("Anna");
    }
}
