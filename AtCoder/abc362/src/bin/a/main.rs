use proconio::input;

fn main() {
    input! {
        r: u32,
        g: u32,
        b: u32,
        c: String,
    };

    let ans = if c == "Red" {
        g.min(b)
    } else if c == "Green" {
        r.min(b)
    } else {
        r.min(g)
    };

    println!("{}", ans);
}
