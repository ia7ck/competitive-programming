use join::Join;
use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    println!("1");
    let mut a = vec![1_u64, 1];
    for i in 1..n {
        let mut b = vec![1];
        for j in 1..i {
            b.push(a[j - 1] + a[j]);
        }
        b.push(1);
        a = b;
        println!("{}", a.iter().join(" "));
    }
}
