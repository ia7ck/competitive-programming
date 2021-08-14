use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();

    let ans = if n <= 125 {
        4
    } else if n <= 211 {
        6
    } else {
        8
    };
    println!("{}", ans);
}
