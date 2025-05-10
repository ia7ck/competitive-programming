use proconio::input;

fn main() {
    input! {
        n: usize,
        m: u32,
        mut a: [u32; n],
    };

    let mut ans = 0;
    while (1..=m).all(|x| a.contains(&x)) {
        ans += 1;
        a.pop().unwrap();
    }
    println!("{}", ans);
}
