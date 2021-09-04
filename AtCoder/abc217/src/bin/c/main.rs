use join::Join;
use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let p: Vec<usize> = rd.get_vec(n);

    let p: Vec<usize> = p.into_iter().map(|x| x - 1).collect();
    let mut q = vec![0; n];
    for i in 0..n {
        q[p[i]] = i;
    }
    println!("{}", q.iter().map(|x| x + 1).join(" "));
}
