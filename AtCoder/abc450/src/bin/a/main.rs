use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let ans = (1..=n).rev().collect::<Vec<_>>();
    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
}
