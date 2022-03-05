use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let n = scan!(usize);
    let s = scan!(String);

    let s: Vec<char> = s.chars().collect();

    let mut a = 0;
    let mut b = 0;
    let mut ans = String::new();
    for i in 0..n {
        match s[i] {
            'A' => {
                a += 1;
            }
            'B' => {
                b += 1;
            }
            'C' => {
                a += b / 2;
                b %= 2;
                ans.push_str("A".repeat(a).as_str());
                ans.push_str("B".repeat(b).as_str());
                ans.push('C');
                a = 0;
                b = 0;
            }
            _ => unreachable!(),
        }
    }
    a += b / 2;
    b %= 2;
    ans.push_str("A".repeat(a).as_str());
    ans.push_str("B".repeat(b).as_str());

    println!("{}", ans);
}

// BABABA
// ABBABA
// ...
// AAAAB
