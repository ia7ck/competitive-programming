use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [u32; n],
    };

    if n == 1 {
        println!("0");
        return;
    }

    let max = p[1..].iter().max().copied().unwrap();
    if p[0] > max {
        println!("0");
    } else {
        println!("{}", max - p[0] + 1);
    }
}
