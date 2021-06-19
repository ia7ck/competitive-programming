use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: u64 = rd.get();

    let mut total = 0;
    for d in 1..=n {
        total += d;
        if total >= n {
            println!("{}", d);
            return;
        }
    }
    unreachable!();
}
