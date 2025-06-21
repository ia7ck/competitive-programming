use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [Usize1; q],
    };

    let mut black = vec![false; n];
    let mut ans = 0;
    for a in a {
        if black[a] {
            if (a >= 1 && black[a - 1]) && (a + 1 < n && black[a + 1]) {
                ans += 1;
            } else if (a >= 1 && black[a - 1]) || (a + 1 < n && black[a + 1]) {
            } else {
                assert!(ans >= 1);
                ans -= 1;
            }
        } else {
            if (a >= 1 && black[a - 1]) && (a + 1 < n && black[a + 1]) {
                assert!(ans >= 1);
                ans -= 1;
            } else if (a >= 1 && black[a - 1]) || (a + 1 < n && black[a + 1]) {
            } else {
                ans += 1;
            }
        }
        black[a] = !black[a];

        println!("{}", ans);
    }
}
