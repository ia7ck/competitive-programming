use lowest_common_ancestor::LowestCommonAncestor;
use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let q: usize = rd.get();

    let mut g = vec![vec![]; n];
    for _ in 0..(n - 1) {
        let a: usize = rd.get();
        let b: usize = rd.get();
        g[a - 1].push(b - 1);
        g[b - 1].push(a - 1);
    }
    let lca = LowestCommonAncestor::new(&g);
    for _ in 0..q {
        let c: usize = rd.get();
        let d: usize = rd.get();
        let dist = lca.get_dist(c - 1, d - 1);
        if dist % 2 == 0 {
            println!("Town");
        } else {
            println!("Road");
        }
    }
}
