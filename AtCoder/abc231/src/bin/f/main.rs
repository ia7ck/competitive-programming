use coordinate_compression::CoordinateCompression;
use fenwick_tree::FenwickTree;
use input_i_scanner::InputIScanner;
use std::iter::FromIterator;

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

    let n = scan!(usize);
    let a = scan!(u32; n);
    let b = scan!(u32; n);

    let com_a = CoordinateCompression::from_iter(a.iter().copied());
    let com_b = CoordinateCompression::from_iter(b.iter().copied());
    let mut bs = vec![vec![]; com_a.size()];
    for i in 0..n {
        bs[com_a.find_index(&a[i])].push(com_b.find_index(&b[i]));
    }
    for i in 0..bs.len() {
        bs[i].sort();
        bs[i].reverse();
    }
    let mut tree = FenwickTree::new(com_b.size(), 0usize);
    let mut ans = 0;
    for bs in bs {
        let mut j = 0;
        while j < bs.len() {
            let mut dup = 1;
            while j  + 1< bs.len() && bs[j] == bs[j + 1] {
                j += 1;
                dup += 1;
            }
            tree.add(bs[j], dup);
            ans += dup * tree.sum(bs[j]..com_b.size());
            j += 1;
        }
    }
    println!("{}", ans);
}
