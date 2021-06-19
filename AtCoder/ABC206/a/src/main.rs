use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: u32 = rd.get();

    let x = n * 108 / 100;
    let y = 206;
    if x < y {
        println!("Yay!");
    } else if x == y {
        println!("so-so");
    } else {
        println!(":(");
    }
}
