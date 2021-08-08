use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let a: u32 = rd.get();
    let b: u32 = rd.get();
    println!("{}", a ^ b);
}
