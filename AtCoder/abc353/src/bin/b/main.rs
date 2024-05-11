use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };

    let mut ans = 0;
    let mut x = 0;
    for a in a {
        if x + a <= k {
            x += a;
        } else {
            ans += 1;
            x = a;
        }
    }
    if x > 0 {
        ans += 1;
    }
    println!("{}", ans);
}
