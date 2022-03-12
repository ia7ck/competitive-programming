use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let (mut v, a, b, c) = scan!((u32, u32, u32, u32));
    loop {
        if v < a {
            println!("F");
            return;
        }
        v -= a;
        if v < b {
            println!("M");
            return;
        }
        v -= b;
        if v < c {
            println!("T");
            return;
        }
        v -= c;
    }
    unreachable!();
}
