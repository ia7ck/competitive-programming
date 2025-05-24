use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    };

    let mut ans = 0;
    for x in 0..=407 {
        // diff(x, a / b) < diff(ans, a / b)
        if (x * b).abs_diff(a) < (ans * b).abs_diff(a) {
            ans = x;
        }
    }
    println!("{}", ans);
}
