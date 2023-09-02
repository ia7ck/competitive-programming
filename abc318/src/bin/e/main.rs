use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut positions = vec![vec![]; n + 1];
    for i in 0..n {
        positions[a[i]].push(i);
    }

    let mut ans = 0_usize;
    for pos in positions {
        let mut d = Vec::new();
        for w in pos.windows(2) {
            d.push(w[1] - w[0] - 1);
        }
        for i in 0..d.len() {
            ans += (i + 1) * (d.len() - i) * d[i];
        }
    }
    println!("{}", ans);
}
