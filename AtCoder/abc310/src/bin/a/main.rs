use proconio::input;

fn main() {
    input! {
        n: usize,
        p: u32,
        q: u32,
        d: [u32; n],
    };

    let ans = p.min(q + d.iter().min().unwrap());
    println!("{}", ans);
}
