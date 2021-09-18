use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let s1: String = rd.get();
    let s2: String = rd.get();
    let s3: String = rd.get();
    let t: Vec<char> = rd.get_chars();

    let mut ans = String::new();
    for c in t {
        if c == '1' {
            ans.push_str(&s1);
        } else if c == '2' {
            ans.push_str(&s2);
        } else {
            ans.push_str(&s3);
        }
    }
    println!("{}", ans);
}
