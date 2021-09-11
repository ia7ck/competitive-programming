use procon_reader::ProconReader;
use union_find::UnionFind;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let m: usize = rd.get();
    let mut edges: Vec<(usize, usize, i64)> = (0..m)
        .map(|_| {
            let a: usize = rd.get();
            let b: usize = rd.get();
            let c: i64 = rd.get();
            (a - 1, b - 1, c)
        })
        .collect();

    edges.sort_by_key(|&(_, _, c)| c);
    let mut uf = UnionFind::new(n);
    let mut ans = 0;
    for (a, b, c) in edges {
        if c <= 0 {
            uf.unite(a, b);
        } else {
            if uf.same(a, b) {
                ans += c;
            } else {
                uf.unite(a, b);
            }
        }
    }
    println!("{}", ans);
}
