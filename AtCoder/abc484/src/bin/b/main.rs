use proconio::input;

fn main() {
    input! {
        k: usize,
        a: [u8; k],
    };

    if !a.contains(&9) {
        println!("-1");
        return;
    }

    let ans = a.iter().min().unwrap();
    println!("{}", ans);
}
