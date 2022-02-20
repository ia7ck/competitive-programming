use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let (a, b) = scan!((usize, usize));
    if a + 1 == b || (a == 1 && b == 10) {
        println!("Yes");
    } else {
        println!("No");
    }
}
