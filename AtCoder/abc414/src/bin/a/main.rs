use proconio::input;

fn main() {
    input! {
        n: usize,
        l: u32,
        r: u32,
        xy: [(u32, u32); n],
    };

    let mut ans = 0;
    for (x, y) in xy {
        if x <= l && r <= y {
            ans += 1;
        }
    }

    println!("{}", ans);
}
