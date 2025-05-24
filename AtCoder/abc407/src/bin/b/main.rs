use proconio::input;

fn main() {
    input! {
        x: u32,
        y: u32,
    };

    let mut meet = 0;
    for a in 1..=6 {
        for b in 1..=6 {
            if a + b >= x || a.abs_diff(b) >= y {
                meet += 1;
            }
        }
    }

    let ans = meet as f64 / 36.0;
    println!("{}", ans);
}
