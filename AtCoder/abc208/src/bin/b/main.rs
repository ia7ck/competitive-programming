use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let p: usize = rd.get();
    let mut fact = vec![1; 11];
    for i in 2..=10 {
        fact[i] = fact[i - 1] * i;
    }
    let mut p = p;
    let mut ans = 0;
    while p > 0 {
        for &f in fact.iter().rev() {
            if p >= f {
                p -= f;
                ans += 1;
                break;
            }
        }
    }
    println!("{}", ans);
}
