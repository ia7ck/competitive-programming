use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        xy: [(usize, usize); q],
    };

    a.sort_unstable();
    let mut next = vec![0; n];
    next[n - 1] = a[n - 1] + 1;
    for i in (0..(n - 1)).rev() {
        if a[i] == a[i + 1] {
            next[i] = next[i + 1];
        } else {
            next[i] = a[i] + 1;
        }
    }
    for (x, y) in xy {
        let mut ng = 0;
        let mut ok = usize::MAX / 3;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            // mid - x - (...) >= y
            let p = a.partition_point(|&a| a <= mid);
            let q = a.partition_point(|&a| a < x);
            if mid >= x && mid + 1 >= y + x + (p - q) {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        println!("{}", ok);
    }
}
