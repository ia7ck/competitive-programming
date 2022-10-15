use proconio::input;

fn main() {
    input! {
        mut x: i64,
        k: i64,
    };

    for i in 0..k {
        let t = 10_i64.pow((i + 1) as u32);
        let (y1, y2) = (x / t * t, (x / t + 1) * t);
        if (y1 - x).abs() < (y2 - x).abs() {
            x = y1;
        } else {
            x = y2;
        }
    }
    println!("{}", x);
}
