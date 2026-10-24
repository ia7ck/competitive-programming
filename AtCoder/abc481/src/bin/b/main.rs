use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i32,
        a: [i32; n],
    };

    let mut ans = 0;
    for i in 0..(n - 1) {
        // [i, i+1)
        if (a[i] <= k && k < a[i + 1]) || (a[i] >= k && k > a[i + 1]) {
            ans += 1;
        }
    }
    if a[n - 1] == k {
        ans += 1;
    }

    println!("{}", ans);
}
