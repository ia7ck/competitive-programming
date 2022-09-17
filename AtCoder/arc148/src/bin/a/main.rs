use proconio::input;

fn gcd(x: u32, y: u32) -> u32 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
    };

    a.sort();

    let mut g = 0;
    for w in a.windows(2) {
        g = gcd(g, w[1] - w[0]);
    }

    if g == 1 {
        println!("2");
    } else {
        println!("1");
    }
}
