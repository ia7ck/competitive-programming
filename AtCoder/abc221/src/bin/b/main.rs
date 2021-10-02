use input_i_scanner::{scan_with, InputIScanner};

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let s = scan_with!(_i_i, String);
    let t = scan_with!(_i_i, String);

    if s == t {
        println!("Yes");
        return;
    }
    let mut s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();
    for i in 0..(s.len()-1) {
        s.swap(i, i + 1);
        if s == t {
            println!("Yes");
            return;
        }
        s.swap(i, i + 1);
    }
    println!("No");
}
