use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        f: [Usize1; n],
    };

    let ans1 = {
        let mut f_sorted = f.clone();
        f_sorted.sort_unstable();
        f_sorted.dedup();
        f_sorted.len() == n
    };

    let ans2 = (0..m).all(|i| f.contains(&i));

    println!("{}", yes_no(ans1));
    println!("{}", yes_no(ans2));
}

fn yes_no(cond: bool) -> &'static str {
    if cond { "Yes" } else { "No" }
}
