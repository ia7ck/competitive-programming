use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut e = vec![0_u64; n + 1];
    for a in 1..=n {
        for b in 1..=n {
            if a * b > n {
                break;
            }
            e[a * b] += 1;
        }
    }

    let mut ans = 0;
    for ab in 1..=n {
        ans += e[ab] * e[n - ab];
    }
    println!("{}", ans);
}
