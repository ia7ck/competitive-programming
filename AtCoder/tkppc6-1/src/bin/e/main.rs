use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let a: Vec<i64> = rd.get_vec(n);

    let mut max_and = 0;
    for i in (0..32).rev() {
        let b = max_and ^ (1 << i);
        let count = a.iter().filter(|&&x| (b & x) == b).count();
        if count >= 2 {
            max_and = b;
        }
    }
    println!("{}", max_and * 2);
}
