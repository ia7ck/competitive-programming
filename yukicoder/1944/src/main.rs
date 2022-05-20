use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i128,
        y: i128,
        r: [i128; n],
    };

    if n == 1 {
        if x * x + y * y == r[0] * r[0] {
            println!("Yes");
        } else {
            println!("No");
        }
        return;
    }

    let mut r = r;
    r.sort();
    let mut d = r[0];
    for &r in &r[1..] {
        d += r * 2;
    }
    if x * x + y * y <= d * d {
        println!("Yes");
    } else {
        println!("No");
    }
}
