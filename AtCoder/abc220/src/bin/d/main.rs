use input_i_scanner::{scan_with, InputIScanner};

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let n = scan_with!(_i_i, usize);
    let a = scan_with!(_i_i, usize; n);

    let m = 998244353;
    let mut dp = vec![0u64; 10];
    dp[a[0]] = 1;
    for i in 1..n {
        let mut nxt = vec![0; 10];
        for j in 0..10 {
            let f = (j + a[i]) % 10;
            let g = (j * a[i]) % 10;
            nxt[f] += dp[j];
            nxt[f] %= m;
            nxt[g] += dp[j];
            nxt[g] %= m;
        }
        dp = nxt;
    }
    for ans in dp {
        println!("{}", ans);
    }
}
