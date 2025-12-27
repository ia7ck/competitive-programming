use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        b: [u64; n],
        c: [u64; n],
    };

    let mut ab = vec![0; n];
    ab[1] = a[0] + b[1];
    let mut a_sum = a[0] + a[1];
    for i in 2..n {
        ab[i] = (ab[i - 1] + b[i]).max(a_sum + b[i]);
        a_sum += a[i];
    }

    let mut ans = 0;
    let mut c_sum = 0;
    for i in (2..n).rev() {
        c_sum += c[i];
        ans = ans.max(ab[i - 1] + c_sum);
    }
    println!("{}", ans);
}
