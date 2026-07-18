use proconio::input;

fn main() {
    input! {
        n: usize,
        abs: [(u32, u32, String); n],
    };

    // x
    let mut rest = 10_000;
    let mut y = 10_000;
    for (a, b, s) in abs {
        rest -= b;
        y -= b;
        if s == "take" {
            rest += b - a;
        }
        y += b - a;
    }

    println!("{}", y - rest);
}
