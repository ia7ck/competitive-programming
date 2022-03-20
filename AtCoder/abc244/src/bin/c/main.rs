use scanner_proc_macro::insert_scanner;
use std::collections::HashSet;
use std::io::{self, Write};

#[insert_scanner]
fn main() {
    let n = scan!(usize);

    let mut stdout = io::stdout();
    let mut query = |x| {
        println!("{}", x);
        stdout.flush().unwrap();
    };

    let mut remain = HashSet::new();
    for x in 1..=(n * 2 + 1) {
        remain.insert(x);
    }
    loop {
        let mut found = false;
        for x in 1..=(n * 2 + 1) {
            if let Some(x) = remain.take(&x) {
                found = true;
                query(x);
                break;
            }
        }
        assert!(found);
        let y = scan!(usize);
        if y == 0 {
            break;
        }
        let success = remain.remove(&y);
        assert!(success);
    }
}
