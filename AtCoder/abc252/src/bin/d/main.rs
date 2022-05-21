use binary_search_range::BinarySearchRange;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };

    let mut a = a;
    a.sort();
    let mut ans = 0;
    for &x in &a {
        let less = a.range(0..x);
        let greater = a.range((x + 1)..std::u32::MAX);
        ans += less.len() * greater.len();
    }
    println!("{}", ans);
}
