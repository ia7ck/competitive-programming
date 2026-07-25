use proconio::input;

use crate::next_permutation::NextPermutation;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: [usize; n],
    };

    let mut ans = 0;
    let mut ord = (1..=n).collect::<Vec<_>>();
    loop {
        if p < ord && ord < q {
            ans += 1;
        }
        if ord.next_permutation() == false {
            break;
        }
    }

    println!("{}", ans);
}

// Bundled
#[rustfmt::skip]
#[allow(unused)]
mod next_permutation {
    pub trait NextPermutation {
        fn next_permutation(&mut self) -> bool;
    }

    impl<T: Ord> NextPermutation for [T] {
        fn next_permutation(&mut self) -> bool {
            if self.len() <= 1 {
                return false;
            }
            let mut i = self.len() - 1;
            while i > 0 && self[i - 1] >= self[i] {
                i -= 1;
            }
            if i == 0 {
                return false;
            }
            let mut j = self.len() - 1;
            while self[i - 1] >= self[j] {
                j -= 1;
            }
            self.swap(i - 1, j);
            self[i..].reverse();
            true
        }
    }
}
