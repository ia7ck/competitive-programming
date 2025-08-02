use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [u32; n],
        b: [u32; m],
    };

    for b in b {
        if let Some(p) = a.iter().position(|&x| x == b) {
            a.remove(p);
        }
    }

    println!(
        "{}",
        a.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
