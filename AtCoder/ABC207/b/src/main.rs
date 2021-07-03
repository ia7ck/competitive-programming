use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let a: u64 = rd.get();
    let b: u64 = rd.get();
    let c: u64 = rd.get();
    let d: u64 = rd.get();

    if b >= c * d {
        println!("-1");
        return;
    }
    let ans = (a + (c * d - b) - 1) / (c * d - b);
    println!("{}", ans);
}

// a + b * k <= c * k * d
// a <= (c * d - b) * k
