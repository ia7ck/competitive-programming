use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [usize; n],
    };

    let mut ans = 0;
    for i in 1..=n {
        for j in 1..=d[i - 1] {
            let mut digits = format!("{}{}", i, j).chars().collect::<Vec<_>>();
            digits.sort();
            digits.dedup();
            if digits.len() == 1 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
