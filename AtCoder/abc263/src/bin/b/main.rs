use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n - 1],
    };

    let mut q = vec![0];
    for p in p {
        q.push(p - 1);
    }

    let mut x = n - 1;
    let mut ans = 0;
    while x > 0 {
        x = q[x];
        ans += 1;
    }
    println!("{}", ans);
}
