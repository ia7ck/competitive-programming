use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
        b: [u32; n - 1],
    };

    a.sort();

    let f = |x: u32| -> bool {
        let mut b = b.clone();
        b.push(x);
        b.sort();
        a.iter().zip(&b).all(|(a, b)| a <= b)
    };

    if !f(1_000_000_000) {
        println!("-1");
        return;
    }

    let mut ng = 0;
    let mut ok = 1_000_000_000;
    while ok - ng > 1 {
        let mid = (ok + ng) / 2;
        if f(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}
