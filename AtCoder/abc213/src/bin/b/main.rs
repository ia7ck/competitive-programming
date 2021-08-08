use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let a: Vec<u32> = rd.get_vec(n);

    let mut ai: Vec<(u32, usize)> = a.iter().copied().enumerate().map(|(i, x)| (x, i)).collect();
    ai.sort();
    ai.reverse();
    let (_, i) = ai[1];
    println!("{}", i + 1);
}
