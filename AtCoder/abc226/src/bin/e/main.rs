use input_i_scanner::InputIScanner;
use union_find::UnionFind;

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    macro_rules! scan {
        (($($t: ty),+)) => {
            ($(scan!($t)),+)
        };
        ($t: ty) => {
            _i_i.scan::<$t>() as $t
        };
        (($($t: ty),+); $n: expr) => {
            std::iter::repeat_with(|| scan!(($($t),+))).take($n).collect::<Vec<_>>()
        };
        ($t: ty; $n: expr) => {
            std::iter::repeat_with(|| scan!($t)).take($n).collect::<Vec<_>>()
        };
    }

    let (n, m) = scan!((usize, usize));
    let mut edges = Vec::new();
    for _ in 0..m {
        let (u, v) = scan!((usize, usize));
        edges.push((u - 1, v - 1));
    }
    let mut uf = UnionFind::new(n);
    for &(u, v) in &edges {
        uf.unite(u, v);
    }
    let mut node_count = vec![0; n];
    let mut edge_count = vec![0; n];
    for u in 0..n {
        node_count[uf.find(u)] += 1;
    }
    for &(u, _) in &edges {
        edge_count[uf.find(u)] += 1;
    }
    let mo: u64 = 998244353;
    let mut ans = 1;
    for u in 0..n {
        if node_count[u] >= 1 {
            if edge_count[u] == node_count[u] {
                ans = ans * 2 % mo;
            } else {
                println!("0");
                return;
            }
        }
    }
    println!("{}", ans);
}
