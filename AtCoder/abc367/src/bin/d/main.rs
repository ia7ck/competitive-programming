use proconio::input;
use binary_search_range::BinarySearchRange;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    };

    let a = a
        .iter()
        .copied()
        .chain(a.iter().copied())
        .collect::<Vec<_>>();

    let mut pos = vec![vec![]; m];
    let mut cumsum = vec![0; a.len() + 1];
    pos[0].push(0);
    for i in 0..a.len() {
        cumsum[i + 1] = (cumsum[i] + a[i]) % m;
        pos[cumsum[i + 1]].push(i + 1);
    }

    let mut ans = 0_usize;
    for start in 0..n {
        let end_range = pos[cumsum[start]].range((start + 1)..(start + n));
        ans += end_range.len();
    }

    println!("{}", ans);
}
