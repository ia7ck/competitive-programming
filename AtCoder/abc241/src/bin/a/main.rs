use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let a = scan!(usize; 10);

    let mut cur = 0;
    for _ in 0..3 {
        cur = a[cur];
    }
    println!("{}", cur);
}
