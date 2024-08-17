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

    let (n, q) = scan!((usize, usize));
    let lr = scan!((usize, usize); q);

    let mut uf = UnionFind::new(n + 1);
    for (l, r) in lr {
        uf.unite(l - 1, r);
    }
    if uf.same(0, n) {
        println!("Yes");
    } else {
        println!("No");
    }
}
