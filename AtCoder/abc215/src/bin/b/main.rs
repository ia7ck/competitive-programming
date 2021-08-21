use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: u64 = rd.get();

    let mut k = 0;
    let mut p2 = 1;
    while p2 * 2 <= n {
        p2 *= 2;
        k += 1;
    }
    println!("{}", k);
}
