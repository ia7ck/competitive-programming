use proconio::{
    input,
    marker::{Chars, Usize1},
};
use segment_tree::SegmentTree;

const M: u64 = 999999937; // prime
const BASE: u64 = 123456789; // < M

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
    };

    let mut seg = SegmentTree::new(
        n,
        (0, 0, 0),
        |&(hash_front_1, hash_back_1, len_1), &(hash_front_2, hash_back_2, len_2)| {
            (
                (hash_front_1 + mpow(BASE, len_1, M) * hash_front_2) % M,
                (hash_back_1 * mpow(BASE, len_2, M) + hash_back_2) % M,
                len_1 + len_2,
            )
        },
    );

    for i in 0..n {
        let h = u64::from(s[i]) % M;
        seg.update(i, (h, h, 1));
    }

    for _ in 0..q {
        input! {
            op: u8,
        };
        if op == 1 {
            input! {
                x: Usize1,
                c: char,
            };
            let h = u64::from(c) % M;
            seg.update(x, (h, h, 1));
        } else {
            input! {
                l: Usize1,
                r: Usize1,
            };
            let (h_front, h_back, _) = seg.fold(l..(r + 1));
            if h_front == h_back {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}

fn mpow(a: u64, x: u64, m: u64) -> u64 {
    if x == 0 {
        1 % m
    } else if x % 2 == 0 {
        mpow(a * a % m, x / 2, m)
    } else {
        a * mpow(a, x - 1, m) % m
    }
}
