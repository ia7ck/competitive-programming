use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: u32 = rd.get();
    let k: u32 = rd.get();

    let mut ans = 0;
    for i in 1..=n {
        for j in 1..=k {
            ans += i * 100 + j;
        }
    }
    println!("{}", ans);
}
