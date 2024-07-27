use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [i64; n],
    };

    a.sort();

    for _ in 0..q {
        input! {
            b: i64,
            k: usize,
        };

        let p = a.partition_point(|&x| x < b);

        let mut ok = 1_000_000_000;
        let mut ng = -1;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            let left = a[..p].partition_point(|&x| b - x > mid);
            let right = a[p..].partition_point(|&x| x - b <= mid);
            if (p - left) + right >= k {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        println!("{}", ok);
    }
}
