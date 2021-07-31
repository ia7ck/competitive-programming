use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let s: String = rd.get();
    let mut xs = Vec::new();
    for c in s.chars() {
        xs.push(c as u8 - '0' as u8);
    }
    if xs.iter().all(|&x| xs[0] == x) {
        println!("Weak");
        return;
    }
    let weak = (0..3).all(|i| (xs[i] + 1) % 10 == xs[i + 1]);
    if weak {
        println!("Weak");
        return;
    }

    println!("Strong");
}
