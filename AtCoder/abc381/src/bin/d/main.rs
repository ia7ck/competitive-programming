use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    };

    let mut ans = 0;
    let mut count = vec![0; n];
    let mut l = 0;
    let mut r = 0;
    // (l, r]
    while r + 1 < n {
        if a[r] == a[r + 1] {
            let x = a[r];
            count[x] += 2;
            r += 2;
            while count[x] != 2 {
                count[a[l]] -= 1;
                l += 1;
            }
        } else {
            let x = a[r];
            count[x] += 1;
            r += 1;
            while l < r && count[x] != 2 {
                count[a[l]] -= 1;
                l += 1;
            }
        }
        ans = ans.max(r - l);
    }

    println!("{}", ans);
}
