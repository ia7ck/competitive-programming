use proconio::input;

fn main() {
    input! {
        n: i64,
    };

    let mut ans = 0;
    for i in 1..=n {
        ans += (-1_i64).pow(i as u32) * i.pow(3);
    }
    println!("{}", ans);
}
