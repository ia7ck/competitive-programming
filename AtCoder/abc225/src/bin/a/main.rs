use input_i_scanner::{scan_with, InputIScanner};
use std::collections::HashSet;

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let s = scan_with!(_i_i, String);
    let s: Vec<char> = s.chars().collect();

    let mut set = HashSet::new();
    set.insert((s[0], s[1], s[2]));
    set.insert((s[0], s[2], s[1]));
    set.insert((s[1], s[0], s[2]));
    set.insert((s[1], s[2], s[0]));
    set.insert((s[2], s[0], s[1]));
    set.insert((s[2], s[1], s[0]));
    println!("{}", set.len());
}
