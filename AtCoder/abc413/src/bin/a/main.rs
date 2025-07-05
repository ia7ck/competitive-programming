use proconio::input;

fn main() {
    input! {
        n: usize,
        m: u32,
        a: [u32; n],
    };

    let sum = a.iter().sum::<u32>();
    if sum <= m {
        println!("Yes");
    } else {
        println!("No");
    }
}
