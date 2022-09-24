use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        mut a: [usize; n],
    };

    let mut heap = BinaryHeap::new();
    for i in 0..n {
        heap.push((Reverse(a[i]), Reverse(i)));
    }
    let mut sub = 0;
    while let Some((Reverse(x), Reverse(i))) = heap.pop() {
        if (x - sub) * (heap.len() + 1) > k {
            // もどす
            heap.push((Reverse(x), Reverse(i)));
            break;
        }
        k -= (x - sub) * (heap.len() + 1);
        sub += x - sub;
        a[i] = 0;
    }

    let l = heap.len();
    let mut indicies = Vec::new();
    for (_, Reverse(i)) in heap {
        indicies.push(i);
    }
    indicies.sort();
    for (t, &i) in indicies.iter().enumerate() {
        a[i] -= sub;
        a[i] -= k / l;
        if t < k % l {
            a[i] -= 1;
        }
    }

    for i in 0..n {
        print!("{}", a[i]);
        if i + 1 < n {
            print!(" ");
        }
    }
    println!();
}
