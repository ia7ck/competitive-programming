use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [u32; n],
    };

    let mut b = vec![vec![]; k];
    for i in 0..n {
        b[i % k].push(a[i]);
    }
    for i in 0..k {
        b[i].sort();
    }
    let mut c = vec![0; n];
    for i in 0..k {
        for (j, &x) in b[i].iter().enumerate() {
            c[i + j * k] = x;
        }
    }
    for w in c.windows(2) {
        if w[0] > w[1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
