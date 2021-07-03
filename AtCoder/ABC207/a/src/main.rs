use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let a: Vec<u32> = rd.get_vec(3);

    let mut a = a;
    a.sort();
    let ans = a[1..].iter().sum::<u32>();
    println!("{}", ans);
}
