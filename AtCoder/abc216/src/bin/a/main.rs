use join::Join;
use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let s: Vec<char> = rd.get_chars();
    let n = s.len();

    let x = s[..(n - 2)].iter().join("");
    match s[n - 1] {
        '0' | '1' | '2' => {
            println!("{}-", x);
        }
        '3' | '4' | '5' | '6' => {
            println!("{}", x);
        }
        _ => {
            println!("{}+", x);
        }
    }
}
