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

    let t = scan!(usize);
    for _ in 0..t {
        let n = scan!(usize);
        let a = scan!(u32; n);
        let b = scan!(u32; n);
        solve(n, a, b);
    }
}

fn solve(n: usize, a: Vec<u32>, b: Vec<u32>) {
    let mut uf = UnionFind::new(n + 1);
    for i in 2..=n {
        for j in ((i + i)..=n).step_by(i) {
            uf.unite(i, j);
        }
    }
    let mut c = vec![vec![]; n + 1];
    for i in 1..=n {
        let p = uf.find(i);
        c[p].push(i);
    }
    for i in 1..=n {
        if c[i].is_empty() {
            continue;
        }
        let mut a: Vec<u32> = c[i].iter().copied().map(|j| a[j - 1]).collect();
        let mut b: Vec<u32> = c[i].iter().copied().map(|j| b[j - 1]).collect();
        a.sort();
        b.sort();
        if a != b {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
