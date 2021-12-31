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
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        let (a, b) = scan!((usize, usize));
        g[a - 1].push(b - 1);
        g[b - 1].push(a - 1);
    }

    let mut uf = UnionFind::new(n);
    let mut ans = Vec::new();
    let mut conn = 0;
    for i in (0..n).rev() {
        ans.push(conn);
        conn += 1;
        for &j in &g[i] {
            if j < i {
                continue;
            }
            if !uf.same(i, j) {
                conn -= 1;
            }
            uf.unite(i, j);
        }
    }
    ans.reverse();
    for ans in ans {
        println!("{}", ans);
    }
}
