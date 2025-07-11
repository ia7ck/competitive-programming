use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut pos = vec![vec![]; n + 1];
    for i in 1..=n {
        pos[i].push(0);
    }
    for i in 0..n {
        pos[a[i]].push(i + 1);
    }
    for i in 1..=n {
        pos[i].push(n + 1);
    }

    let mut ans = 0;
    for i in 1..=n {
        let total = n * (n + 1) / 2;
        let mut sub = 0;
        for w in pos[i].windows(2) {
            let len = w[1] - w[0] - 1;
            sub += len * (len + 1) / 2;
        }
        ans += total - sub;
    }
    println!("{}", ans);
}
