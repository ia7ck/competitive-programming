use join::Join;
use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = 26;
    let p: Vec<usize> = rd.get_vec(n);

    let mut chars = Vec::new();
    for i in 0..n {
        let c = ('a' as usize + i) as u8 as char;
        chars.push(c);
    }
    println!("{}", p.iter().map(|&p| chars[p - 1]).join(""));
}
