use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        k: u64,
    };

    let mut a = a;
    let mut ans = 0;
    while a < b {
        a *= k;
        ans += 1;
    }
    println!("{}", ans);
}
