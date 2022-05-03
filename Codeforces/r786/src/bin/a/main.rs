use proconio::input;

fn main() {
    input! {
        t: usize,
    };
    for _ in 0..t {
        input! {
            x: u32,
            y: u32,
        };
        solve(x, y);
    }
}

fn solve(x: u32, y: u32) {
    for b in 1..=100 {
        let mut z = 1;
        for a in 1..=100 {
            z *= b;
            if x * z == y {
                println!("{} {}", a, b);
                return;
            }
            if x * z > y {
                break;
            }
        }
    }
    println!("0 0");
}
