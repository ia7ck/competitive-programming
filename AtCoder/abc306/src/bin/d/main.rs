use proconio::{input};

fn main() {
    input! {
        n: usize,
        xy: [(u8, i64); n],
    };

    let mut dp_good = 0;
    let mut dp_bad = std::i64::MIN / 2;
    for (x, y) in xy {
        let (good, bad) = if x == 0 {
            (dp_good.max(dp_good + y).max(dp_bad + y), dp_bad)
        } else {
            (dp_good, (dp_good + y).max(dp_bad))
        };
        dp_good = good;
        dp_bad = bad;
    }

    println!("{}", dp_good.max(dp_bad));
}
