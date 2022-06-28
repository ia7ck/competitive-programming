use proconio::input;

fn main() {
    input! {
        mut a: [u64; 3],
    };

    a.sort();
    if a[0] + a[1] >= a[2] {
        println!("{}", a[2]);
    } else {
        println!("-1");
    }
}
