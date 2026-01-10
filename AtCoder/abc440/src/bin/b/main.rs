use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [u32; n],
    };

    let mut ord = (0..n).collect::<Vec<_>>();
    ord.sort_by_key(|&i| t[i]);
    let ans = &ord[..3];

    println!(
        "{}",
        ans.iter()
            .map(|x| (x + 1).to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
