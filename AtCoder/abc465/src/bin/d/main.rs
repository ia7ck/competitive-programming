use proconio::input;

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            x: i64,
            y: i64,
            k: i64,
        };

        solve(x, y, k);
    }
}

fn solve(x: i64, y: i64, k: i64) {
    let mut ans = 0;
    let mut x = x;
    let mut y = y;
    while x != y {
        if x > y {
            x /= k;
        } else {
            y /= k;
        }
        ans += 1;
    }

    println!("{}", ans);
}
