use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    };

    let mut q = vec![0; n];
    for j in 0..n {
        q[p[j]] = j;
    }

    let mut a = vec![0; n];

    for i in 0..n {
        if q[i] < i {
            a[(i - q[i] + n - 1) % n] += 1;
            a[(i - q[i]) % n] += 1;
            a[(i - q[i] + 1) % n] += 1;
        } else {
            a[(n - (q[i] - i) - 1) % n] += 1;
            a[(n - (q[i] - i)) % n] += 1;
            a[(n - (q[i] - i) + 1) % n] += 1;
        }
    }

    let ans = a.iter().max().unwrap();
    println!("{}", ans);
}
