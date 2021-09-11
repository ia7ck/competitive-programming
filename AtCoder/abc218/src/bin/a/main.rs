use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let s: Vec<char> = rd.get_chars();

    if s[n - 1] == 'o' {
        println!("Yes");
    } else {
        println!("No");
    }
}
