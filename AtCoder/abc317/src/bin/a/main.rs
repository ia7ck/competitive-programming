use proconio::input;

fn main() {
    input! {
        n: usize,
        h: u32,
        x: u32,
        p: [u32; n],
    };

    for i in 0..n {
        if h + p[i] >= x {
            println!("{}", i + 1);
            return;
        }
    }

    unreachable!();
}
