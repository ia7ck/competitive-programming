use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let a: f64 = rd.get();
    let b: f64 = rd.get();

    let ans = (a * b) / 100.0;
    println!("{}", ans);
}
