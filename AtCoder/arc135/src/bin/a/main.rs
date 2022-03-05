use scanner_proc_macro::insert_scanner;
use std::collections::HashMap;

fn f(x: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if x <= 4 {
        return x;
    }
    if let Some(&ans) = memo.get(&x) {
        return ans;
    }
    let ans = f(x / 2, memo) * f((x + 1) / 2, memo) % 998244353;
    memo.insert(x, ans);
    return ans;
}

#[insert_scanner]
fn main() {
    let x = scan!(u64);
    let mut memo = HashMap::new();
    let ans = f(x, &mut memo);
    println!("{}", ans);
}
