use proconio::input;

fn main() {
    input! {
        n: usize,
        d: u64,
        x: [u64; n],
    };

    let mut ans = Vec::new();
    for i in 0..n {
        if (0..n).all(|j| j == i || x[i].abs_diff(x[j]) >= d) {
            ans.push(i);
        }
    }

    println!("{}", ans.len());
    for ans in ans {
        println!("{}", ans + 1);
    }
}
