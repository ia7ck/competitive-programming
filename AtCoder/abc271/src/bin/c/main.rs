use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut ok = 0;
    let mut ng = a.len() + 1;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        let mut b = vec![false; mid + 1];
        b[0] = true;
        let mut r = 0;
        for &x in &a {
            if x <= mid {
                if b[x] == false {
                    b[x] = true;
                } else {
                    r += 1;
                }
            } else {
                r += 1;
            }
        }
        let q = b.iter().filter(|&&f| f == false).count();
        if r / 2 >= q {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}
