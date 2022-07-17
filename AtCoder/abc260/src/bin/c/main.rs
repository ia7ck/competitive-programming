use proconio::input;

fn solve(n: usize, is_red: bool, x: u64, y: u64) -> (u64, u64) {
    if n == 1 {
        if is_red {
            return (1, 0);
        } else {
            return (0, 1);
        }
    }
    if is_red {
        let (r, b) = solve(n - 1, true, x, y);
        let (rr, bb) = solve(n, false, x, y);
        (r + rr * x, b + bb * x)
    } else {
        let (r, b) = solve(n - 1, true, x, y);
        let (rr, bb) = solve(n - 1, false, x, y);
        (r + rr * y, b + bb * y)
        
    }
}

fn main() {
    input! {
        n: usize,
        x: u64,
        y: u64,
    };

    let (_, b) = solve(n, true, x, y);
    println!("{}", b);
}
