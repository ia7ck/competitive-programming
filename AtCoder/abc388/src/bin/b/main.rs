use proconio::input;

fn main() {
    input! {
        n: usize,
        d: u32,
        snakes: [(u32, u32); n],
    };

    for k in 1..=d {
        let ans = snakes.iter().map(|(t, l)| t * (l + k)).max().unwrap();
        println!("{}", ans);
    }
}
