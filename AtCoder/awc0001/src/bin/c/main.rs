use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut d: [u64; n],
    };

    d.sort_unstable();
    d.reverse();

    let ans = d.into_iter().skip(k).sum::<u64>();

    println!("{}", ans);
}
