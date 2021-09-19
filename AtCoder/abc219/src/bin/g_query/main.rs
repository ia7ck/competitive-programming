use join::Join;
use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let m: usize = rd.get();
    let q: usize = rd.get();
    let mut gg = vec![vec![]; n];
    for _ in 0..m {
        let u: usize = rd.get();
        let v: usize = rd.get();
        gg[u - 1].push(v - 1);
        gg[v - 1].push(u - 1);
    }
    let xs: Vec<usize> = (0..q)
        .map(|_| {
            let x: usize = rd.get();
            x - 1
        })
        .collect();

    let mut seen = vec![false; n];
    let mut g = vec![vec![]; n];
    let mut last = Vec::new();
    for v in 0..n {
        last.push((0, v));
    }
    let mut color: Vec<usize> = (0..n).collect();
    for xs in xs.chunks(5000) {
        for v in 0..n {
            seen[v] = false;
        }
        for &x in xs {
            seen[x] = true;
        }
        for v in 0..n {
            g[v].clear();
        }
        for u in 0..n {
            if seen[u] {
                for &v in &gg[u] {
                    if seen[v] {
                        g[u].push(v);
                    }
                }
            }
        }
        for v in 0..n {
            last[v] = (0, color[v]);
        }
        for (i, &x) in xs.iter().enumerate() {
            for &y in &g[x] {
                color[y] = color[x];
            }
            last[x] = (i + 1, color[x]);
        }
        for u in 0..n {
            if !seen[u] {
                let mut m = (0, color[u]);
                for &v in &gg[u] {
                    if seen[v] {
                        m = m.max(last[v]);
                    }
                }
                color[u] = m.1;
            }
        }
    }
    println!("{}", color.iter().map(|c| c + 1).join(" "));
}
