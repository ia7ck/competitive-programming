use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u32,
        a: [u64; n],
    };

    let mut x = 1_u64;
    for a in a {
        x = x.saturating_mul(a);
        if x >= 10_u64.pow(k) {
            x = 1;
        }
    }

    println!("{}", x);
}
