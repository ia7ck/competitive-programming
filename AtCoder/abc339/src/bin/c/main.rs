use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };

    let mut cur = 0;
    let mut bottom = 0;
    for &x in &a {
        cur += x;
        bottom = bottom.min(cur);
    }

    let ans = cur + -1 * bottom;
    println!("{}", ans);
}
