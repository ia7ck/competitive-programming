use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
    };

    a.sort();
    a.dedup();
    assert!(a.len() >= 2);

    let ans = a[a.len() - 2];
    println!("{}", ans);
}
