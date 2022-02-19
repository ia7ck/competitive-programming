use scanner_proc_macro::insert_scanner;
use std::collections::HashSet;

#[insert_scanner]
fn main() {
    let (x1, y1) = scan!((i64, i64));
    let (x2, y2) = scan!((i64, i64));

    let dydx = vec![
        (-1, -2),
        (-2, -1),
        (-2, 1),
        (-1, 2),
        (1, 2),
        (2, 1),
        (2, -1),
        (1, -2),
    ];
    let mut set1 = HashSet::new();
    let mut set2 = HashSet::new();
    for (dy, dx) in dydx {
        set1.insert((x1 + dx, y1 + dy));
        set2.insert((x2 + dx, y2 + dy));
    }
    if set1.intersection(&set2).count() >= 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
