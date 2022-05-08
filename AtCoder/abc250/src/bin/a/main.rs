use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        r: usize,
        c: usize,
    };

    let mut ans = 0;
    if r - 1 >= 1 {
        ans += 1;
    }
    if c - 1 >= 1 {
        ans += 1;
    }
    if r + 1 <= h {
        ans += 1;
    }
    if c + 1 <= w {
        ans += 1;
    }
    println!("{}", ans);
}
