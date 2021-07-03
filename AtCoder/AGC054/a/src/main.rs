use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let s: Vec<char> = rd.get_chars();

    if s[0] != s[n - 1] {
        println!("1");
        return;
    }

    for i in 0..(n - 1) {
        if s[0] != s[i] && s[i + 1] != s[n - 1] {
            println!("2");
            return;
        }
    }

    println!("-1");
}
