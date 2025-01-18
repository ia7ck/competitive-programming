use proconio::input;

fn main() {
    input! {
        r: u64,
    };

    let inside = {
        let mut ans = 0;
        for x in 1..r {
            // (ok+0.5) <= sqrt(r^2 - (x+0.5)^2)
            let mut ok = 0;
            let mut ng = r + 1;
            while ng - ok > 1 {
                let mid = (ng + ok) / 2;
                if (mid * 2 + 1).pow(2) <= r.pow(2) * 4 - (x * 2 + 1).pow(2) {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ans += ok;
        }
        ans
    };
    let line = r - 1;
    let ans = inside * 4 + line * 4 + 1;
    println!("{}", ans);
}
