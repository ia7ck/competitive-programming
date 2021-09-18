use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let x: u32 = rd.get();
    if x < 40 {
        println!("{}", 40 - x);
    } else if x < 70 {
        println!("{}", 70 - x);
    } else if x < 90 {
        println!("{}", 90 - x);
    } else {
        println!("expert");
    }
}
