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
    let mut deg = vec![0; n];
    let mut edges = Vec::new();
    for _ in 0..m {
        let (a, b) = scan!((usize, usize));
        deg[a - 1] += 1;
        deg[b - 1] += 1;
        edges.push((a - 1, b - 1));
    }

    let ng = deg.iter().any(|&d| d > 2);
    if ng {
        println!("No");
        return;
    }
    let mut uf = UnionFind::new(n);
    for (a, b) in edges {
        if uf.same(a, b) {
            println!("No");
            return;
        }
        uf.unite(a, b);
    }
    println!("Yes");
}
