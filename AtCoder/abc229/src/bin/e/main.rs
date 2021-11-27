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
    let edges = scan!((usize, usize); m);

    let mut g = vec![vec![]; n];
    for (a, b) in edges {
        g[a - 1].push(b - 1);
        g[b - 1].push(a - 1);
    }
    let mut uf = UnionFind::new(n);
    let mut con = 0usize;
    let mut ans = Vec::new();
    for u in (0..n).rev() {
        ans.push(con);
        con += 1;
        for &v in &g[u] {
            if v > u {
                if !uf.same(u, v) {
                    uf.unite(u, v);
                    con -= 1;
                }
            }
        }
    }

    ans.reverse();
    for ans in ans {
        println!("{}", ans);
    }
}
