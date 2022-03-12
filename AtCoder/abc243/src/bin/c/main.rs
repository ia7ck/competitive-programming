use scanner_proc_macro::insert_scanner;
use std::collections::HashMap;

#[insert_scanner]
fn main() {
    let n = scan!(usize);
    let xy = scan!((u32, u32); n);
    let s = scan!(String);
    let s: Vec<char> = s.chars().collect();

    let mut r_min = HashMap::new();
    let mut l_max = HashMap::new();
    for i in 0..n {
        let (x, y) = xy[i];
        if s[i] == 'R' {
            let e = r_min.entry(y).or_insert(x);
            *e = (*e).min(x);
        } else {
            let e = l_max.entry(y).or_insert(x);
            *e = (*e).max(x);
        }
    }
    for (y, x) in r_min {
        if let Some(&xx) = l_max.get(&y) {
            if x < xx {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
