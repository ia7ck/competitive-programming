use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let a: Vec<usize> = rd.get_vec(n);

    let p: Vec<usize> = (1..=n).collect();
    let mut a = a;
    a.sort();
    let ans = if a == p { "Yes" } else { "No" };
    println!("{}", ans);
}
