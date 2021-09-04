use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let s: String = rd.get();
    let t: String = rd.get();
    if s < t {
        println!("Yes");
    } else {
        println!("No");
    }
}
