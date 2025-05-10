use proconio::input;

fn main() {
    input! {
        r: u32,
        x: u8,
    };

    let ans = if x == 1 {
        1600 <= r && r <= 2999
    } else {
        1200 <= r && r <= 2399
    };

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
