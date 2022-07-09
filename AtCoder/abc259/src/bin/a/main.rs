use proconio::input;

fn main() {
    input! {
        _n: u16,
        m: u16,
        x: u16,
        t: u16,
        d: u16,
    };

    if x <= m {
        println!("{}", t);
    } else {
        println!("{}", t - d * (x - m));
    }
}
