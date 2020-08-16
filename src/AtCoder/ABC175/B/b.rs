extern crate proconio;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut ans = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                if a[i] == a[j] {
                    continue;
                }
                if a[j] == a[k] {
                    continue;
                }
                if a[i] == a[k] {
                    continue;
                }

                if a[i] + a[j] > a[k] && a[j] + a[k] > a[i] && a[k] + a[i] > a[j] {
                    // println!("{} {} {}", i + 1, j + 1, k + 1);
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
