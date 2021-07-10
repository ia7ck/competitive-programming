use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let c: Vec<u64> = rd.get_vec(n);

    let mo = 1_000_000_000 + 7;

    let mut c = c;
    c.sort();
    let mut ans = 1;
    for i in 0..n {
        match c[i].checked_sub(i as u64) {
            None => {
                println!("0");
                return;
            }
            Some(x) => {
                ans = ans * x % mo;
            }
        }
    }
    println!("{}", ans);
}
