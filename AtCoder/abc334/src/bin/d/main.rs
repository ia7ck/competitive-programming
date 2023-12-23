use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut r: [u64; n],
        xs: [u64; q],
    };

    r.sort();
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + r[i];
    }
    for x in xs {
        let i = acc.partition_point(|&y| y <= x);
        assert!(i >= 1);
        println!("{}", i - 1);
    }
}
