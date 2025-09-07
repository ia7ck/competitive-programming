use std::ops::Range;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };

    let mut b = vec![0; 1 << n];
    f(0..(1 << n), k, &mut b);
    let u = usize::from(k % (1 << n) != 0);

    println!("{}", u);
    println!(
        "{}",
        b.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn f(range: Range<usize>, k: usize, b: &mut Vec<usize>) {
    if range.start + 1 == range.end {
        b[range.start] = k;
        return;
    }

    let m = (range.start + range.end) / 2;
    let left = k / 2;
    let right = k - k / 2;
    f(range.start..m, left, b);
    f(m..range.end, right, b);
}
