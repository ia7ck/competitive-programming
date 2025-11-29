use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, u32); n],
    };

    let mut b_by_a = vec![vec![]; m];
    for (a, b) in ab {
        b_by_a[a].push(b);
    }

    for k in 0..m {
        assert!(!b_by_a[k].is_empty());
        let total = b_by_a[k].iter().sum::<u32>();
        let ans = total as f64 / b_by_a[k].len() as f64;
        println!("{}", ans);
    }
}
