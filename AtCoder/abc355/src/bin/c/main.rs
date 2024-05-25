use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        a: [usize; t],
    };

    let mut row = vec![0; n + 1];
    let mut col = vec![0; n + 1];
    let mut diag = 0;
    let mut gaid = 0;
    for (i, a) in a.iter().enumerate() {
        let (r, c) = ((a - 1) / n, (a - 1) % n);
        row[r] += 1;
        col[c] += 1;
        if r == c {
            diag += 1;
        }
        if n - r - 1 == c {
            gaid += 1;
        }
        if row[r] == n || col[c] == n || diag == n || gaid == n {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}
