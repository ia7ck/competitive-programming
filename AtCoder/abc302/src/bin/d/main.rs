use proconio::input;
use binary_search_range::BinarySearchRange;

fn main() {
    input! {
        n: usize,
        m: usize,
        d: u64,
        a: [u64; n],
        mut b: [u64; m],
    };

    // A - b <= d
    // b >= A - d

    // -d <= A - b
    // b <= A + d

    b.sort();
    let mut ans = 0;
    for x in a {
        let rng = b.range(x.saturating_sub(d)..(x + d + 1));
        if rng.start < rng.end {
            let y = b[rng.end - 1];
            ans = ans.max(x + y);
        }
    }

    if ans == 0 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
