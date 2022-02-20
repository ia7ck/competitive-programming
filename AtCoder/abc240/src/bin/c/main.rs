use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let (n, x) = scan!((usize, usize));
    let ab = scan!((usize, usize); n);

    let mut dp = vec![false; x + 1];
    dp[0] = true;
    for (a, b) in ab {
        let mut nxt = vec![false; x + 1];
        for y in 0..x {
            if y + a <= x {
                nxt[y + a] |= dp[y];
            }
            if y + b <= x {
                nxt[y + b] |= dp[y];
            }
        }
        dp = nxt;
    }

    if dp[x] {
        println!("Yes");
    } else {
        println!("No");
    }
}
