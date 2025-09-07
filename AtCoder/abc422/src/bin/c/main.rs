use proconio::input;

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            na: usize,
            nb: usize,
            nc: usize,
        };

        solve(na, nb, nc);
    }
}

fn solve(na: usize, nb: usize, nc: usize) {
    let f = |k: usize| -> bool {
        if na < k || nc < k {
            return false;
        }

        let m = (na - k) + nb + (nc - k);
        m >= k
    };

    let mut ok = 0;
    let mut ng = 1_000_000_000 + 1;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        if f(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
