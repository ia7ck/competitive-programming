use std::collections::BTreeMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    };

    let mut tile = BTreeMap::<usize, char>::new();
    let mut untile = BTreeMap::<usize, char>::new();
    let mut last_c = 'a';
    for _ in 0..q {
        input! {
            op: u8,
        };

        if op == 1 {
            input! {
                x: Usize1,
            };

            match (tile.get(&x), untile.get(&x)) {
                (Some(_), Some(_)) => unreachable!(),
                (None, None) => {
                    tile.insert(x, last_c);
                }
                (Some(&t), None) => {
                    tile.remove(&x);
                    untile.insert(x, t);
                }
                (None, Some(&ut)) => {
                    untile.remove(&x);
                    tile.insert(x, ut);
                }
            }
        } else {
            input! {
                c: char,
            };

            untile.clear();
            last_c = c;
        }
    }

    let mut ans = String::new();
    for i in 0..n {
        let c = match (tile.get(&i), untile.get(&i)) {
            (Some(_), Some(_)) => unreachable!(),
            (None, None) => last_c,
            (Some(&t), None) => t,
            (None, Some(&ut)) => ut,
        };
        ans.push(c);
    }
    println!("{}", ans);
}
