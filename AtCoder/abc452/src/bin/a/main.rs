use proconio::input;

fn main() {
    input! {
        m: u32,
        d: u32,
    };

    let ans = match (m, d) {
        (1, 7) => "Yes",
        (3, 3) => "Yes",
        (5, 5) => "Yes",
        (7, 7) => "Yes",
        (9, 9) => "Yes",
        _ => "No",
    };

    println!("{}", ans);
}
