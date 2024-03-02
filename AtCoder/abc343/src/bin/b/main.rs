use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[u8; n]; n],
    };

    for i in 0..n {
        let adj = (0..n).filter(|&j| a[i][j] == 1).collect::<Vec<_>>();
        println!(
            "{}",
            adj.iter()
                .map(|j| (j + 1).to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}
