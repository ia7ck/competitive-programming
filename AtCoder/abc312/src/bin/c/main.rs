use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u32; n],
        b: [u32; m],
    };

    let mut ng = 0;
    let mut ok = 1_000_000_000 + 1;
    while ok - ng > 1 {
        let mid = (ok + ng) / 2;
        let sell = a.iter().filter(|&&x| mid >= x).count();
        let buy = b.iter().filter(|&&x| mid <= x).count();
        if sell >= buy {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}
