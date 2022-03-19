use scanner_proc_macro::insert_scanner;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[insert_scanner]
fn main() {
    let (l, r) = scan!((u64, u64));
    let mut ans = 1;
    for x in l..(l + 1000).min(r) {
        for y in (x + 1).max(r.saturating_sub(1000))..=r {
            if gcd(x, y) == 1 {
                ans = ans.max(y - x);
            }
        }
    }
    println!("{}", ans);
}
