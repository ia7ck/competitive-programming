use std::collections::{BinaryHeap, HashSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: usize,
        mut a: [i64; n],
    };

    a.sort_unstable();
    a.reverse();

    #[derive(Debug, PartialEq, Eq)]
    struct S {
        sum: i64,
        choice: Vec<usize>,
    }

    let new_s = |choice: Vec<usize>| -> S {
        S {
            sum: a.iter().zip(&choice).map(|(&a, &c)| a * c as i64).sum(),
            choice,
        }
    };

    impl Ord for S {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.sum.cmp(&other.sum)
        }
    }
    impl PartialOrd for S {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut ans = Vec::new();
    let mut initial = vec![0; n];
    initial[0] = k;
    let mut seen = HashSet::new();
    let mut heap = BinaryHeap::new();
    seen.insert(initial.clone());
    heap.push(new_s(initial));
    loop {
        let S { sum, choice } = heap.pop().unwrap();
        ans.push(sum);
        if ans.len() == x {
            break;
        }

        for i in 0..(n - 1) {
            if choice[i] > 0 {
                let mut new_choice = choice.clone();
                new_choice[i] -= 1;
                new_choice[i + 1] += 1;
                if !seen.contains(&new_choice) {
                    seen.insert(new_choice.clone());
                    heap.push(new_s(new_choice));
                }
            }
        }
    }

    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    )
}
