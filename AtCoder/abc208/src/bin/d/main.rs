use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let m: usize = rd.get();
    let inf = std::u64::MAX / 2;
    let mut d = vec![vec![inf; n]; n];
    for v in 0..n {
        d[v][v] = 0;
    }
    for _ in 0..m {
        let a: usize = rd.get();
        let b: usize = rd.get();
        let c: u64 = rd.get();
        d[a - 1][b - 1] = c;
    }

    let mut ans = 0;
    for k in 0..n {
        for s in 0..n {
            for t in 0..n {
                d[s][t] = d[s][t].min(d[s][k] + d[k][t]);
            }
        }
        for s in 0..n {
            for t in 0..n {
                if d[s][t] < inf {
                    ans += d[s][t];
                }
            }
        }
    }
    println!("{}", ans);
}
