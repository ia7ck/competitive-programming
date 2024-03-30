use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u32,
        a: [u32; n],
    };

    let ans = a
        .iter()
        .filter(|&&x| x % k == 0)
        .map(|&x| x / k)
        .collect::<Vec<_>>();
    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
