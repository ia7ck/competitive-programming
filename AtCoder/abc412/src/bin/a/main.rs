use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(u32, u32); n],
    };

    let mut ans = 0;
    for (a, b) in ab {
        if a < b {
            ans += 1;
        }
    }

    println!("{}", ans);
}
