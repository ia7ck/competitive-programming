use std::collections::HashSet;
use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let n = scan!(usize);
    let a = scan!(u32; n);

    let mut set = HashSet::new();
    for a in a {
        set.insert(a);
    }
    println!("{}", set.len());
}
