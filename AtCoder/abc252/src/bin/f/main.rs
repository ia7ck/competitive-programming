use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        l: u64,
        a: [u64; n],
    };

    let sum = a.iter().sum::<u64>();
    let mut heap = BinaryHeap::new();
    for a in a {
        heap.push(Reverse(a));
    }
    if sum < l {
        heap.push(Reverse(l - sum));
    }
    let mut ans = 0;
    while heap.len() >= 2 {
        let Reverse(min) = heap.pop().unwrap();
        let Reverse(second_min) = heap.pop().unwrap();
        ans += min + second_min;
        heap.push(Reverse(min + second_min));
    }
    let Reverse(last) = heap.pop().unwrap();
    assert_eq!(l, last);
    println!("{}", ans);
}
