use input_i_scanner::{scan_with, InputIScanner};

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let s = scan_with!(_i_i, String);

    let mut ans_min = s.clone();
    let mut ans_max = s.clone();
    let mut s: Vec<char> = s.chars().collect();
    for _ in 0..s.len() {
        s.reverse();
        let c = s.pop().unwrap();
        s.reverse();
        s.push(c);
        let t: String = s.iter().collect();
        ans_min = ans_min.min(t.clone());
        ans_max = ans_max.max(t);
    }
    for _ in 0..s.len() {
        let c = s.pop().unwrap();
        s.reverse();
        s.push(c);
        s.reverse();
        let t: String = s.iter().collect();
        ans_min = ans_min.min(t.clone());
        ans_max = ans_max.max(t);
    }
    println!("{}", ans_min);
    println!("{}", ans_max);
}
