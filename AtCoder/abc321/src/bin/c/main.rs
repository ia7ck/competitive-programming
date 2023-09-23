use proconio::input;

fn solve(cur: u64, values: &mut Vec<u64>) {
    values.push(cur);
    for d in 0..=9 {
        match (cur, d) {
            (0, 0) => {}
            (0, d) => {
                solve(d, values);
            }
            (cur, d) if cur % 10 > d => {
                solve(cur * 10 + d, values);
            }
            _ => {}
        }
    }
}

fn main() {
    input! {
        k: usize,
    };

    let mut values = Vec::new();
    solve(0, &mut values);
    values.sort();
    assert!(k < values.len());
    let ans = values[k];
    println!("{}", ans);
}
