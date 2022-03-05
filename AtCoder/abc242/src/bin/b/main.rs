use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let s = scan!(String);
    let mut s: Vec<char> = s.chars().collect();

    s.sort();
    for ch in s {
        print!("{}", ch);
    }
    println!();
}
