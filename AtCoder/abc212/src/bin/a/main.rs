use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let a: u32 = rd.get();
    let b: u32 = rd.get();

    if 0 < a && b == 0 {
        println!("Gold");
        return;
    }
    if a == 0 && 0 < b {
        println!("Silver");
        return;
    }
    if 0 < a && 0 < b {
        println!("Alloy");
        return;
    }

    unreachable!();
}
