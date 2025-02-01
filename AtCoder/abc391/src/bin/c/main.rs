use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    };

    let mut pos = (0..n).collect::<Vec<_>>();
    let mut count = vec![1; n];
    let mut ans = 0;
    for _ in 0..q {
        input! {
            op: u8,
        };
        if op == 1 {
            input! {
                p: Usize1,
                h: Usize1,
            };
            assert!(count[pos[p]] >= 1);
            count[pos[p]] -= 1;
            if count[pos[p]] == 1 {
                assert!(ans >= 1);
                ans -= 1;
            }
            pos[p] = h;
            count[pos[p]] += 1;
            if count[pos[p]] == 2 {
                ans += 1;
            }
        } else {
            println!("{}", ans);
        }
    }
}
