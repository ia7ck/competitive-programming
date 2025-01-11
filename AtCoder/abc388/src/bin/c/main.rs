use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n], 
    };

    let mut ans = 0;
    for i in 0..n {
        let p = a[..i].partition_point(|&x| x <= a[i] / 2);
        ans += p;
    }
    println!("{}", ans);
}
