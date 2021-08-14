use procon_reader::ProconReader;
use union_find::UnionFind;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let mut edges = Vec::new();
    for _ in 0..(n - 1) {
        let u: usize = rd.get();
        let v: usize = rd.get();
        let w: u64 = rd.get();
        edges.push((u - 1, v - 1, w));
    }

    edges.sort_by_key(|&(_, _, w)| w);
    let mut uf = UnionFind::new(n);
    let mut ans = 0;
    for (u, v, w) in edges {
        let us = uf.get_size(u) as u64;
        let vs = uf.get_size(v) as u64;
        ans += w * us * vs;
        uf.unite(u, v);
    }
    println!("{}", ans);
}
