use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let (n, m) = scan!((usize, usize));
    let c = (0..n).map(|_| scan!(usize; m)).collect();
    solve(n, m, c);
}

fn solve(n: usize, m: usize, c: Vec<Vec<usize>>) {
    let k = 1_000_00;
    let mut cnt = vec![0; k + 1];
    let mut d_sum = vec![0; k + 1];
    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            ans += i * cnt[c[i][j]] - d_sum[c[i][j]];
        }
        for j in 0..m {
            cnt[c[i][j]] += 1;
            d_sum[c[i][j]] += i;
        }
    }
    let mut cnt = vec![0; k + 1];
    let mut d_sum = vec![0; k + 1];
    for j in 0..m {
        for i in 0..n {
            ans += j * cnt[c[i][j]] - d_sum[c[i][j]];
        }
        for i in 0..n {
            cnt[c[i][j]] += 1;
            d_sum[c[i][j]] += j;
        }
    }
    println!("{}", ans);
}
