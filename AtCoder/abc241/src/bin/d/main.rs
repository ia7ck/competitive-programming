use scanner_proc_macro::insert_scanner;
use std::collections::BTreeSet;

#[insert_scanner]
fn main() {
    let q = scan!(usize);

    let mut set = BTreeSet::new();
    for i in 0..q {
        let (tp, x) = scan!((u8, u64));
        match tp {
            1 => {
                set.insert((x, i));
            }
            2 => {
                let k = scan!(usize);
                if let Some((ans, _)) = set.range(..&(x, std::usize::MAX)).rev().nth(k - 1) {
                    println!("{}", ans);
                } else {
                    println!("-1");
                }
            }
            3 => {
                let k = scan!(usize);
                if let Some((ans, _)) = set.range((x, 0)..).nth(k - 1) {
                    println!("{}", ans);
                } else {
                    println!("-1");
                }
            }
            _ => unreachable!(),
        }
    }
}
