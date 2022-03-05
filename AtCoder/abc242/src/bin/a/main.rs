use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let (a, b, c, x) = scan!((u32, u32, u32, u32));

    if x <= a {
        println!("1");
        return;
    }
    if x > b {
        println!("0");
        return;
    }
    assert!(a < x && x <= b);
    let ans = c as f64 / (b - a) as f64;
    println!("{}", ans);
}
