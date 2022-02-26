use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let (n, m) = scan!((usize, usize));
    let mut a = scan!(u32; n);
    let b = scan!(u32; m);

    for b in b {
        if let Some(index) = a.iter().position(|&a| a == b) {
            a.swap_remove(index);
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
