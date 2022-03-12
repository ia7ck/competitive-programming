use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let (n, m) = scan!((usize, usize));
    let inf = std::u64::MAX / 2;
    let mut d = vec![vec![inf; n]; n];
    for i in 0..n {
        d[i][i] = 0;
    }
    let mut edges = Vec::new();
    for _ in 0..m {
        let (a, b, c) = scan!((usize, usize, u64));
        let (a, b) = (a - 1, b - 1);
        d[a][b] = c;
        d[b][a] = c;
        edges.push((a, b, c));
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                d[i][j] = d[i][j].min(d[i][k] + d[k][j]);
            }
        }
    }
    let mut ans = 0;
    for (a, b, c) in edges {
        let rm = if d[a][b] < c {
            true
        } else {
            assert_eq!(d[a][b], c);
            let mut alt = false;
            for v in 0..n {
                if v == a || v == b {
                    continue;
                }
                if d[a][v] + d[v][b] == c {
                    alt = true;
                }
            }
            alt
        };
        if rm {
            ans += 1;
        }
    }
    println!("{}", ans);
}
