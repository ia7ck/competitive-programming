use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(u64, u64); n],
    };

    let a_sum = ab.iter().map(|(a, _)| a).sum::<u64>();
    let mut ans = 0;
    for (a, b) in ab {
        ans = ans.max(a_sum - a + b);
    }
    println!("{}", ans);
}
