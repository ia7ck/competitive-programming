use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        x: i32,
        y: i32,
        a: [i32; n],
    };

    let mut dp_x = HashSet::new();
    dp_x.insert(a[0]);
    let mut dp_y = HashSet::new();
    dp_y.insert(0);
    for i in 1..n {
        if i % 2 == 1 {
            let mut new_dp_y = HashSet::new();
            for y in dp_y {
                new_dp_y.insert(y + a[i]);
                new_dp_y.insert(y - a[i]);
            }
            dp_y = new_dp_y;
        } else {
            let mut new_dp_x = HashSet::new();
            for x in dp_x {
                new_dp_x.insert(x + a[i]);
                new_dp_x.insert(x - a[i]);
            }
            dp_x = new_dp_x;
        }
    }

    let ok = dp_x.contains(&x) && dp_y.contains(&y);
    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
