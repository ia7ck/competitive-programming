use proconio::input;

fn main() {
    input! {
        l: u8,
        r: u8,
    };

    let ans = match (l, r) {
        (1, 0) => "Yes",
        (0, 1) => "No",
        _ => "Invalid",
    };

    println!("{}", ans);
}
