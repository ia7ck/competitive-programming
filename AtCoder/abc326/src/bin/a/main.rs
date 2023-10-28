use proconio::input;

fn main() {
    input! {
        x: u16,
        y: u16,
    };

    let ans = if x < y {
        if y <= x + 2 {
            "Yes"
        } else {
            "No"
        }
    } else {
        if x <= y + 3 {
            "Yes"
        } else {
            "No"
        }
    };

    println!("{}", ans);
}
