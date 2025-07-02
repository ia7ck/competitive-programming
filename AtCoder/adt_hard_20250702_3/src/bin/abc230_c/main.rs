use proconio::input;

fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
        p: i64,
        q: i64,
        r: i64,
        s: i64,
    };

    for i in p..=q {
        let mut ans = String::new();
        for j in r..=s {
            // (i, j) = (a + k, b + k)
            // or
            // (i, j) = (a + k, b - k)

            let mut black = false;
            if i - a == j - b {
                let k = i - a;
                if (1 - a).max(1 - b) <= k && k <= (n - a).min(n - b) {
                    black = true;
                }
            }
            if i - a == b - j {
                let k = i - a;
                if (1 - a).max(b - n) <= k && k <= (n - a).min(b - 1) {
                    black = true;
                }
            }
            if black {
                ans.push('#');
            } else {
                ans.push('.');
            }
        }
        println!("{}", ans);
    }
}
