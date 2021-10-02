use fenwick_tree::FenwickTree;
use input_i_scanner::{scan_with, InputIScanner};
use mod_int::ModInt998244353;

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let n = scan_with!(_i_i, usize);
    let a = scan_with!(_i_i, u32; n);

    let mut ai: Vec<(u32, usize)> = a.into_iter().enumerate().map(|(i, x)| (x, i)).collect();
    ai.sort();

    type Mint = ModInt998244353;
    let mut f = FenwickTree::new(n, Mint::new(0));
    let mut ans = Mint::new(0);
    let mut pows = vec![Mint::new(1); n + 1];
    for i in 1..=n {
        pows[i] = pows[i - 1] * 2;
    }
    for (_, i) in ai {
        let denom = f.sum(0..i);
        ans += pows[i] * denom;
        f.add(i, pows[i + 1].inv());
    }
    println!("{}", ans.val());
}
