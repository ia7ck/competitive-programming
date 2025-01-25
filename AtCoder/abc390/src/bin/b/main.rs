use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    };

    // r := a[1] / a[0]
    for i in 2..n {
        // a[i] / a[i - 1] == r
        if a[i] * a[0] == a[1] * a[i - 1] {
            // ok
        } else {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
