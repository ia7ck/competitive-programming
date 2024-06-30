use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
        w: [u32; n],
    };

    let mut b = vec![vec![]; n];
    for i in 0..n {
        b[a[i]].push(w[i]);
    }

    let mut ans = 0;
    for i in 0..n {
        b[i].sort();
        if b[i].len() >= 2 {
            ans += b[i][..b[i].len() - 1].iter().sum::<u32>();
        }
    }
    println!("{}", ans);
}
