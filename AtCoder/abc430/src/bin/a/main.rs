use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
        d: u32,
    };

    let ans = if c >= a { d < b } else { false };
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
