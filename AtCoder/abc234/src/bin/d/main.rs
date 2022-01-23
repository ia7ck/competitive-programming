use input_i_scanner::InputIScanner;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

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

    let (n, k) = scan!((usize, usize));
    let a = scan!(u32; n);

    let mut ans = Vec::new();
    let mut heap = BinaryHeap::new();
    for i in 0..k {
        heap.push(Reverse(a[i]));
    }
    ans.push(heap.peek().copied().unwrap());
    for i in k..n {
        heap.push(Reverse(a[i]));
        heap.pop();
        ans.push(heap.peek().copied().unwrap());
    }
    for Reverse(ans) in ans {
        println!("{}", ans);
    }
}
