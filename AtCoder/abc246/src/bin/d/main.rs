use scanner_proc_macro::insert_scanner;

fn f(a: u64, b: u64) -> u64 {
    a * a * a + a * a * b + a * b * b + b * b * b
}

#[insert_scanner]
fn main() {
    let n = scan!(u64);

    let mut a = 0;
    while a * a * a < n {
        a += 1;
    }

    let inf = std::u64::MAX;
    let mut ans = inf;
    for b in 0..=n {
        let b3 = b * b * b;
        if b3 >= n {
            // a = 0
            ans = ans.min(b3);
            break;
        }
        while f(a, b) >= n {
            ans = ans.min(f(a, b));
            a = a.saturating_sub(1);
            if a == 0 {
                break;
            }
        }
    }

    assert_ne!(ans, inf);
    println!("{}", ans);
}
