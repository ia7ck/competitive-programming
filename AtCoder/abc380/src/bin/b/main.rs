use proconio::input;

fn main() {
    input! {
        s: String,
    };

    let a = s
        .trim_matches('|')
        .split('|')
        .map(|a| a.len())
        .collect::<Vec<_>>();
    println!(
        "{}",
        a.iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
