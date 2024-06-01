use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    };

    let mut a = vec![vec![]; m];
    let mut r = vec!['?'; m];
    for i in 0..m {
        input! {
            c: usize,
            a_: [Usize1; c],
            r_: char,
        };
        a[i] = a_;
        r[i] = r_;
    }

    let mut ans = 0;
    for bits in 0..(1 << n) {
        let ok = (0..m).all(|i| {
            let correct = a[i].iter().filter(|&&j| bits >> j & 1 == 1).count();
            if r[i] == 'o' {
                correct >= k
            } else {
                correct < k
            }
        });
        if ok {
            ans += 1;
        }
    }

    println!("{}", ans);
}
