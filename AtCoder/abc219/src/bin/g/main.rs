use join::Join;
use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let m: usize = rd.get();
    let q: usize = rd.get();
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        let u: usize = rd.get();
        let v: usize = rd.get();
        g[u - 1].push(v - 1);
        g[v - 1].push(u - 1);
    }
    let xs: Vec<usize> = rd.get_vec(q);

    let mut deg = vec![0; n];
    for u in 0..n {
        for &v in &g[u] {
            deg[v] += 1;
        }
    }
    const B: usize = 500;
    let mut large = vec![false; n];
    for v in 0..n {
        if deg[v] >= B {
            large[v] = true;
        }
    }
    let mut lg = vec![vec![]; n];
    for u in 0..n {
        if large[u] {
            for &v in &g[u] {
                if large[v] {
                    lg[u].push(v);
                }
            }
        }
    }
    let mut color: Vec<usize> = (0..n).collect();
    let mut changed_at = vec![None; n];
    for (i, x) in xs.into_iter().enumerate() {
        let x = x - 1;
        if large[x] {
            for &y in &lg[x] {
                color[y] = color[x];
            }
            changed_at[x] = Some((i, color[x]));
        } else {
            let mut ixs = vec![changed_at[x]];
            for &y in &g[x] {
                ixs.push(changed_at[y]);
            }
            let c = ixs
                .into_iter()
                .filter_map(|o| o)
                .max()
                .map(|(_, c)| c)
                .unwrap_or(x);
            for &y in &g[x] {
                if large[y] {
                    color[y] = c;
                }
            }
            changed_at[x] = Some((i, c));
        }
    }
    // eprintln!("{:?}", changed_at);
    let mut ans = Vec::new();
    for u in 0..n {
        if large[u] {
            ans.push(color[u]);
        } else {
            let mut ixs = vec![changed_at[u]];
            for &v in &g[u] {
                ixs.push(changed_at[v]);
            }
            let c = ixs
                .into_iter()
                .filter_map(|o| o)
                .max()
                .map(|(_, c)| c)
                .unwrap_or(u);
            ans.push(c);
        }
    }
    println!("{}", ans.iter().map(|ans| ans + 1).join(" "));
}
