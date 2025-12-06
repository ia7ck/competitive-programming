use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };

    let mut ans = 0;
    for l in 0..n {
        for r in l..n {
            let sum = a[l..=r].iter().sum::<u32>();
            if (l..=r).all(|i| sum % a[i] != 0) {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
