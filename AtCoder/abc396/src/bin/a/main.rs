use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };

    for i in 0..(n - 2) {
        if a[i] == a[i + 1] && a[i + 1] == a[i + 2] {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
