use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let x = scan!(i64);
    let ans = if x >= 0 {
        x / 10
    } else if x % 10 == 0 {
        x / 10
    } else {
        x / 10 - 1
    };
    println!("{}", ans);
}
