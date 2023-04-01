use divisors::Divisors;
use proconio::input;

fn main() {
    input! {
        n: u64,
        m: u64,
    };

    for d in m.divisors() {
        if d <= n && m / d <= n {
            println!("{}", m);
            return;
        }
    }

    let mut ans = std::u64::MAX;
    for k in 1..=n.min(1000000) {
        let l = (m + k - 1) / k;
        assert!(k * l >= m);
        if k <= n && l <= n {
            ans = ans.min(k * l);
        }
    }

    if ans == std::u64::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }

}
