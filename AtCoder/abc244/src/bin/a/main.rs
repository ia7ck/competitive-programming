use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let _n = scan!(usize);
    let s = scan!(String);
    let s: Vec<char> = s.chars().collect();

    let ans = s.iter().last().copied().unwrap();
    println!("{}", ans);
}
