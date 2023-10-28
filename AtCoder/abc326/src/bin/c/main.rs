use binary_search_range::BinarySearchRange;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: u64,
        mut a: [u64; n],
    };

    a.sort();
    let mut ans = 0;
    for &x in &a {
        let rng = a.range(x..(x + m));
        ans = ans.max(rng.len());
    }
    println!("{}", ans);
}
