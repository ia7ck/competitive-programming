use proconio::input;

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: u64,
        };

        solve(n);
    }
}

fn solve(n: u64) {
    let mut ans = 1; // x = y = 1

    for i in 1..=9 {
        let ten = 10_u64.pow(i);
        
        // y = 98, 998, 9998, ...
        if ten.pow(2) - ten * 2 <= n {
            ans += 1;
        }

        // y = 9, 99, 999, 9999, ...
        if ten.pow(2) - ten <= n {
            ans += (ten.pow(2) - 1).min(n) - (ten.pow(2) - ten) + 1;
        }

        // y = 10, 100, 1000, 10000, ...
        if ten.pow(2) <= n {
            ans += (ten.pow(2) + ten - 1).min(n) - ten.pow(2) + 1;
        }
    }

    println!("{}", ans);
}
