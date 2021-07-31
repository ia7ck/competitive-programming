use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let m: usize = rd.get();
    let k: usize = rd.get();
    let mut compl_g = vec![vec![]; n];
    for _ in 0..m {
        let u: usize = rd.get();
        let v: usize = rd.get();
        compl_g[u - 1].push(v - 1);
        compl_g[v - 1].push(u - 1);
    }
    for u in 0..n {
        compl_g[u].push(u);
    }

    let mo: i64 = 998244353;
    let mut ways = vec![0; n];
    ways[0] = 1;
    for _ in 0..k {
        let tot = ways.iter().copied().fold(0, |acc, x| (acc + x) % mo);
        let mut nxt = vec![tot; n];
        for u in 0..n {
            for &v in &compl_g[u] {
                nxt[v] = (mo + nxt[v] - ways[u]) % mo;
            }
        }
        ways = nxt;
    }
    println!("{}", ways[0]);
}
