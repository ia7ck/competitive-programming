use proconio::input;

fn main() {
    input! {
        n: u64,
        m: u32,
    };

    let mut x = 0_u64;
    for i in 0..=m {
        x = x.saturating_add(n.saturating_pow(i));
    }
    if x > 1_000_000_000 {
        println!("inf");
    } else {
        println!("{}", x);
    }
}
