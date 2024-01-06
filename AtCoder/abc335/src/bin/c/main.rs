use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: isize,
        q: isize,
    };

    let mut map = HashMap::new();
    for i in 1..=n {
        map.insert(i, (i as i32, 0));
    }

    let mut count = 0;
    for _ in 0..q {
        input! {
            op: u8,
        };
        if op == 1 {
            input! {
                dir: char,
            };
            let (x, y) = map[&(1 - count)];
            match dir {
                'L' => {
                    map.insert(-count, (x - 1, y));
                }
                'R' => {
                    map.insert(-count, (x + 1, y));
                }
                'U' => {
                    map.insert(-count, (x, y + 1));
                }
                'D' => {
                    map.insert(-count, (x, y - 1));
                }
                _ => unreachable!(),
            }
            count += 1;
        } else {
            input! {
                p: isize,
            };
            let (x, y) = map[&(p - count)];
            println!("{} {}", x, y);
        }
    }
}
