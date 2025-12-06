use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut r = 1 + a[0];
    for i in 1..n {
        if i + 1 < r {
            r = r.max((i + 1) + a[i]);
        } else {
            println!("{}", i);
            return;
        }
    }

    println!("{}", n);
}
