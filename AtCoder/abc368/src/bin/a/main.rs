use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [u32; n],
    };

    a.rotate_right(k);

    println!(
        "{}",
        a.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
