use proconio::{input, marker::Bytes};

fn main() {
    input! {
        s: Bytes,
    };

    let s = s.iter().map(|&b| b - b'0').collect::<Vec<_>>();
    let ans = solve(&s, 0, 0);
    println!("{}", ans);
}

fn solve(s: &[u8], a: usize, b: usize) -> usize {
    match s.split_last() {
        Some((&last, rest)) => solve(
            rest,
            a + 1,
            // b + (last - b) % 10
            b + (10 + usize::from(last) - b % 10) % 10,
        ),
        None => a + b,
    }
}
