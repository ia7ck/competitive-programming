use proconio::{input, marker::Usize1};

use lowest_common_ancestor::LowestCommonAncestor;

fn main() {
    input! {
        n: usize,
        s: Usize1,
        t: Usize1,
        edges: [(Usize1, Usize1); n - 1],
    };

    let lca = LowestCommonAncestor::new(n, s, &edges);
    for j in 0..n {
        let v = lca.get(j, t);
        println!("{}", lca.get_dist(j, v) + 1);
    }
}
