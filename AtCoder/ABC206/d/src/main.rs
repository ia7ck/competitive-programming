use procon_reader::ProconReader;
use union_find::UnionFind;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let a: Vec<usize> = rd.get_vec(n);

    let mut uf = UnionFind::new(2_000_00 + 1);
    let mut ans = 0;
    for i in 0..(n / 2) {
        if !uf.same(a[i], a[n - i - 1]) {
            ans += 1;
            uf.unite(a[i], a[n - i - 1]);
        }
    }
    println!("{}", ans);
}
