use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let (a , b) = scan!((f64, f64));

    let x = a / a.hypot(b);
    let y = b / a.hypot(b);
    println!("{} {}", x, y);
}
