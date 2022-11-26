use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    };

    let mut b = vec![vec![]; n + 1];
    let mut p = 1;
    for i in 0..m {
        b.swap(a[i], a[i] + 1);
        b[p].push(i);
        if a[i] == p {
            p = a[i] + 1;
        } else if a[i] + 1 == p {
            p = a[i];
        }
    }
    let mut ans = vec![0; m];
    for i in 1..=n {
        for &b in &b[i] {
            ans[b] = i;
        }
    }
    for ans in ans {
        println!("{}", ans);
    }
}
