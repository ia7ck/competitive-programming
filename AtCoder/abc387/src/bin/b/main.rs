use proconio::input;

fn main() {
    input! {
        x: u32,
    };

    let mut ans = 0;
    for i in 1..=9 {
        for j in 1..=9 {
            let y = i * j;
            if x != y {
                ans += y;
            }
        }
    }

    println!("{}", ans);
}
