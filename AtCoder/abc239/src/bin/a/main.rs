use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let h = scan!(f64);
    let ans = (h * (12800000.0 + h)).sqrt();
    println!("{}", ans);
}
