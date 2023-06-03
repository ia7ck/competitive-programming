use proconio::input;

fn main() {
    input! {
        n: usize,
        sa: [(String, u32); n],
    };

    let a_min = sa.iter().map(|(_, a)| a).min().copied().unwrap();
    let start = sa.iter().position(|&(_, a)| a == a_min).unwrap();
    for d in 0..n {
        let (s, _) = sa[(start + d) % n].clone();
        println!("{}", s);
    }
}
