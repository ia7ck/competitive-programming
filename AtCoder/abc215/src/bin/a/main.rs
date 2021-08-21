use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let s: String = rd.get();

    if s == "Hello,World!" {
        println!("AC");
    } else {
        println!("WA");
    }
}
