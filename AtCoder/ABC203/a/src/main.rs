use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let a: u32 = rd.get();
    let b: u32 = rd.get();
    let c: u32 = rd.get();

    if a == b {
        println!("{}", c);
    } else if b == c {
        println!("{}", a);
    } else if c == a {
        println!("{}", b);
    } else {
        println!("0");
    }
}
