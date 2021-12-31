use input_i_scanner::InputIScanner;
use segment_tree::SegmentTree;

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
    let a = scan!(usize; n);

    let mut freq = vec![0; n + 1];
    for &a in &a[..m] {
        freq[a] += 1;
    }
    let mut seg = SegmentTree::new(
        n + 1,
        (std::usize::MAX, std::usize::MAX),
        |val_index_l, val_index_r| *val_index_l.min(val_index_r),
    );
    for i in 0..=n {
        seg.update(i, (freq[i], i));
    }
    let (min, index) = seg.fold(0..(n + 1));
    assert_eq!(min, 0);
    let mut ans = index;
    for i in m..n {
        freq[a[i - m]] -= 1;
        seg.update(a[i - m], (freq[a[i - m]], a[i - m]));
        freq[a[i]] += 1;
        seg.update(a[i], (freq[a[i]], a[i]));
        let (min, index) = seg.fold(0..(n + 1));
        assert_eq!(min, 0);
        ans = ans.min(index);
    }
    println!("{}", ans);
}
