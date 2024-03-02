use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    };

    for x in 0..=9 {
        if x != a + b {
            println!("{}", x);
            return;
        }
    }

    unreachable!();
}
