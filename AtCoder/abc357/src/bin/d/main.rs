use mod_int::ModInt998244353;
use proconio::input;

type Mint = ModInt998244353;

fn main() {
    input! {
        n: u64,
    };


    let len = n.to_string().len();
    let r = Mint::new(10).pow(len as u32);
    // n + n * 10^len + n * 10^(len * 2) + ... + n * 10^(len * (n - 1))
    // = n * (1 + 10^len + 10^(len * 2) + ... + 10^(len * (n - 1)))
    // = n * (r^n - 1) / (r - 1)

    let ans = Mint::from(n) * ((pow(r, n) - 1) / (r - 1));

    println!("{}", ans.val());
}

fn pow(a: Mint, x: u64) -> Mint {
    if x == 0 {
        Mint::new(1)
    } else if x % 2 == 0 {
        pow(a * a, x / 2)
    } else {
        a * pow(a, x - 1)
    }
}
