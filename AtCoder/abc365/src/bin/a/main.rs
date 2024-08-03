use proconio::input;

fn main() {
    input! {
        y: u32,
    };

    let ans = if y % 4 != 0 { 
        365
    } else if y % 100 != 0 {
        366
    } else if y % 400 != 0 {
        365
    } else {
        366
    };

    println!("{}", ans);
}
