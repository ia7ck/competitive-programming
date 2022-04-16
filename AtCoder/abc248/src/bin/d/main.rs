use binary_search_range::BinarySearchRange;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        lrx: [(usize, usize, usize); q],
    };

    let mut positions = vec![vec![]; n];
    for i in 0..n {
        positions[a[i] - 1].push(i);
    }
    for i in 0..n {
        positions[i].sort();
    }
    for (l, r, x) in lrx {
        let rng = positions[x - 1].range((l - 1)..r);
        println!("{}", rng.count());
    }
}
