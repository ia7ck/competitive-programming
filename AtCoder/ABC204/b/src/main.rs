use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let a: Vec<u32> = rd.get_vec(n);

    let mut ans = 0;
    for x in a {
        if x >= 10 {
            ans += x - 10;
        }
    }
    println!("{}", ans);
}
