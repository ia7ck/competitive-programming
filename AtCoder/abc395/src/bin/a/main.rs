use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };

    let ans = a.windows(2).all(|w| w[0] < w[1]);
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
