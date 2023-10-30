use proconio::input;

fn main() {
    input! {
        n: usize,
        m: u64,
        mut a: [u64; n],
    };

    a.sort();
    let mut ans = 0;
    for i in 0..n {
        let j = a.partition_point(|&x| x < a[i] + m);
        ans = ans.max(j - i);
    }
    println!("{}", ans);
}
