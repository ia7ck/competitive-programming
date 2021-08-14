use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());
    
    let s: u32 = rd.get();
    let t: u32 = rd.get();
    
    let mut ans = 0_u32;
    for a in 0..=s {
        for b in 0..=s {
            for c in 0..=s {
                if a + b + c <= s && a * b * c <= t {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
