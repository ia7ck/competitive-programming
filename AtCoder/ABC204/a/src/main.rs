use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let x: u32 = rd.get();
    let y: u32 = rd.get();

    let z = if x == y {
        x
    } else {
        3 - x - y
    };

    println!("{}", z);
}
