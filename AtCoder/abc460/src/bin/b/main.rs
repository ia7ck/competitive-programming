use proconio::input;

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            x1: i64,
            y1: i64,
            r1: i64,
            x2: i64,
            y2: i64,
            r2: i64,
        };

        let d = (x1 - x2).pow(2) + (y1 - y2).pow(2);
        let ans = d <= (r1 + r2).pow(2) && d >= (r1 - r2).pow(2);

        if ans {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
