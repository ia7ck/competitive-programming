use input_i_scanner::{scan_with, InputIScanner};

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let n = scan_with!(_i_i, usize);
    let s: Vec<char> = scan_with!(_i_i, String).chars().collect();

    let mut ans = 0;
    for i in 0..(n - 2) {
        if s[i] != 'N' {
            continue;
        }
        if s[i + 1] != 'A' {
            continue;
        }
        for j in (i + 2)..n {
            if s[j] == 'N' {
                ans += 1;
                break;
            }
        }
    }
    println!("{}", ans);
}
