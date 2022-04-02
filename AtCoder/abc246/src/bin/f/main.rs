use mod_int::ModInt998244353;
use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let (n, l) = scan!((usize, usize));
    let s = scan!(String; n);

    let s: Vec<usize> = s
        .into_iter()
        .map(|s| {
            let mut bit = 0;
            for ch in s.chars() {
                bit ^= 1 << (ch as usize - 'a' as usize);
            }
            bit
        })
        .collect();

    type Mint = ModInt998244353;

    let mut ans = Mint::new(0);
    for bits in 1_usize..(1 << n) {
        let mut chars = (1 << 26) - 1;
        for i in 0..n {
            if bits >> i & 1 == 1 {
                chars &= s[i];
            }
        }
        if bits.count_ones() % 2 == 1 {
            ans += Mint::from(chars.count_ones()).pow(l);
        } else {
            ans -= Mint::from(chars.count_ones()).pow(l);
        }
    }
    println!("{}", ans.val());
}
