use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };

    let x = a[0];
    if a.iter().all(|&y| x == y) {
        println!("Yes");
    } else {
        println!("No");
    }
}
