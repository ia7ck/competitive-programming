use mod_int::ModInt998244353;
use prime_factorization::PrimeFactorization;
use floor_sqrt::floor_sqrt;

use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
    };

    let mut x = ModInt998244353::new(1);
    for (_, e) in a.prime_factorization() {
        x *= ModInt998244353::from(b) * e + 1;
    }
    let square = floor_sqrt(a).pow(2) == a;
    let ans = if square && b % 2 == 1 {
        (x * b - 1) / 2
    } else {
        x * b / 2
    };
    println!("{}", ans.val());
}
