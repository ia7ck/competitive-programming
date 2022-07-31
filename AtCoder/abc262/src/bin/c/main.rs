use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut s = 0;
    let mut ans = 0_u64;
    for (j, &x) in a.iter().enumerate() {
        if j + 1 == x {
            // a[i] = i, a[j] = j
            ans += s;
            s += 1;
        } else if j + 1 > x {
            // a[i] = j, a[j] = i
            if a[x - 1] == j + 1 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
