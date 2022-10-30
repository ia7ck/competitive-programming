use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [u32; n],
    };

    let max = h.iter().copied().max().unwrap();
    let ans = h.iter().position(|&x| x == max).unwrap() + 1;
    println!("{}", ans);
}
