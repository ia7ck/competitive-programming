use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        q: usize,
    };
    let mut set = BTreeSet::new();
    for i in 0..q {
        input! {
            op: u8,
        };
        if op == 1 {
            input! {
                x: u32,
            };
            set.insert((x, i));
        } else if op == 2 {
            input! {
                x: u32,
                c: usize,
            };
            let remove = set
                .range((x, 0)..(x, std::usize::MAX))
                .take(c)
                .copied()
                .collect::<Vec<_>>();
            for e in remove {
                set.remove(&e);
            }
        } else {
            let (max, _) = set.iter().last().unwrap();
            let (min, _) = set.iter().next().unwrap();
            println!("{}", max - min);
        }
    }
}
